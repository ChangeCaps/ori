use std::{collections::HashMap, num::NonZeroU32, rc::Rc};

use ori_app::{App, AppBuilder, AppRequest, UiBuilder};
use ori_core::{
    canvas::{Canvas, Color},
    command::CommandWaker,
    event::{Modifiers, PointerButton, PointerId},
    layout::{Point, Size, Vector},
    window::{Window, WindowUpdate},
};
use winit::{
    dpi::LogicalSize,
    event::{Event, KeyEvent, MouseScrollDelta, TouchPhase, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    keyboard::{ModifiersState, PhysicalKey},
    window::WindowBuilder,
};

use crate::{
    clipboard::WinitClipboard,
    convert::{convert_cursor_icon, convert_key, convert_mouse_button, is_pressed},
    render, Error,
};

/// Launch an application.
pub fn launch<T>(app: AppBuilder<T>, data: T) -> Result<(), Error> {
    /* initialize tracing if enabled */
    if let Err(err) = crate::tracing::init_tracing() {
        eprintln!("Failed to initialize tracing: {}", err);
    }

    let event_loop = EventLoop::new()?;

    let waker = CommandWaker::new({
        let proxy = event_loop.create_proxy();

        move || {
            let _ = proxy.send_event(());
        }
    });

    let mut app = app.build(waker);
    app.set_clipboard(WinitClipboard::new());

    let mut state = WinitState::new(data, app);

    event_loop.run(move |event, target| {
        match event {
            // we need to recreate the surfaces when the event loop is resumed
            //
            // this is necessary for android
            Event::Resumed => {
                state.resume();
            }
            Event::AboutToWait => {
                // after all events for a frame have been processed, we need to
                // run the idle function
                state.idle();
            }

            // this event is sent by [`WinitWaker`] telling us that there are
            // commands that need to be processed
            Event::UserEvent(_) => {
                state.app.handle_commands(&mut state.data);
            }
            Event::WindowEvent { window_id, event } => {
                state.window_event(window_id, event);
            }
            _ => {}
        }

        state.handle_requests(target);
    })?;

    Ok(())
}

type RcWindow = Rc<winit::window::Window>;

struct WindowState {
    window: RcWindow,
    #[allow(unused)]
    context: softbuffer::Context<RcWindow>,
    surface: softbuffer::Surface<RcWindow, RcWindow>,
    pixmap: tiny_skia::Pixmap,
    old_canvas: Canvas,
    clear_color: Color,
}

struct WinitState<T> {
    init: bool,
    app: App<T>,
    data: T,
    window_ids: HashMap<winit::window::WindowId, ori_core::window::WindowId>,
    windows: HashMap<ori_core::window::WindowId, WindowState>,
}

impl<T> WinitState<T> {
    fn new(data: T, app: App<T>) -> Self {
        Self {
            init: false,
            app,
            data,
            window_ids: HashMap::new(),
            windows: HashMap::new(),
        }
    }

    fn resume(&mut self) {
        if self.init {
            return;
        }

        self.init = true;
        self.app.init(&mut self.data);
    }

    fn handle_requests(&mut self, target: &EventLoopWindowTarget<()>) {
        for request in self.app.take_requests() {
            if let Err(err) = self.handle_request(target, request) {
                tracing::error!("Failed to handle request: {}", err);
            }
        }
    }

    fn handle_request(
        &mut self,
        target: &EventLoopWindowTarget<()>,
        request: AppRequest<T>,
    ) -> Result<(), Error> {
        match request {
            AppRequest::OpenWindow(desc, builder) => {
                self.create_window(target, desc, builder)?;
            }
            AppRequest::CloseWindow(id) => {
                self.windows.remove(&id);
            }
            AppRequest::DragWindow(id) => {
                if let Some(state) = self.windows.get_mut(&id) {
                    if let Err(err) = state.window.drag_window() {
                        tracing::warn!("Failed to drag window: {}", err);
                    }
                }
            }
            AppRequest::RequestRedraw(id) => {
                if let Some(state) = self.windows.get_mut(&id) {
                    state.window.request_redraw();
                }
            }
            AppRequest::UpdateWindow(id, update) => {
                if let Some(state) = self.windows.get_mut(&id) {
                    match update {
                        WindowUpdate::Title(title) => state.window.set_title(&title),
                        WindowUpdate::Icon(icon) => match icon {
                            Some(icon) => {
                                let icon = winit::window::Icon::from_rgba(
                                    icon.data().to_vec(),
                                    icon.width(),
                                    icon.height(),
                                )
                                .expect("Failed to create icon");

                                state.window.set_window_icon(Some(icon));
                            }
                            None => {
                                state.window.set_window_icon(None);
                            }
                        },
                        WindowUpdate::Size(size) => {
                            let size = size.max(Size::all(10.0));
                            let inner = LogicalSize::new(size.width, size.height);

                            state.window.set_min_inner_size(Some(inner));
                            state.window.set_max_inner_size(Some(inner));
                        }
                        WindowUpdate::Scale(_) => {}
                        WindowUpdate::Resizable(resizable) => {
                            state.window.set_resizable(resizable);
                        }
                        WindowUpdate::Decorated(decorated) => {
                            state.window.set_decorations(decorated);
                        }
                        WindowUpdate::Maximized(maximized) => {
                            state.window.set_maximized(maximized);
                        }
                        WindowUpdate::Visible(visible) => {
                            state.window.set_visible(visible);
                        }
                        WindowUpdate::Color(color) => {
                            let transparent = color.map_or(false, Color::is_translucent);
                            state.window.set_transparent(transparent);
                        }
                        WindowUpdate::Cursor(cursor) => {
                            state.window.set_cursor_icon(convert_cursor_icon(cursor));
                        }
                    }
                }
            }
            AppRequest::Quit => target.exit(),
        }

        Ok(())
    }

    fn idle(&mut self) {
        self.app.idle(&mut self.data);
    }

    fn create_window(
        &mut self,
        target: &EventLoopWindowTarget<()>,
        ori: Window,
        builder: UiBuilder<T>,
    ) -> Result<(), Error> {
        let window_id = ori.id();

        /* create the window */
        let window = WindowBuilder::new()
            .with_title(&ori.title)
            .with_inner_size(LogicalSize::new(ori.width(), ori.height()))
            .with_resizable(ori.resizable)
            .with_decorations(ori.decorated)
            .with_transparent(ori.color.map_or(false, Color::is_translucent))
            .with_visible(false)
            .build(target)?;

        let window = Rc::new(window);

        self.window_ids.insert(window.id(), window_id);

        let icon = match ori.icon {
            Some(ref icon) => {
                let icon = winit::window::Icon::from_rgba(
                    icon.data().to_vec(),
                    icon.width(),
                    icon.height(),
                )
                .expect("Failed to create icon");

                Some(icon)
            }
            None => None,
        };

        window.set_window_icon(icon);
        window.set_visible(ori.visible);
        window.set_maximized(ori.maximized);

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        self.windows.insert(
            window_id,
            WindowState {
                window,
                context,
                surface,
                pixmap: tiny_skia::Pixmap::new(ori.width(), ori.height()).unwrap(),
                old_canvas: Canvas::new(),
                clear_color: Color::TRANSPARENT,
            },
        );

        /* add the window to the ui */
        self.app.add_window(&mut self.data, builder, ori);

        Ok(())
    }

    fn render(&mut self, window_id: ori_core::window::WindowId) -> Result<(), Error> {
        fn pack_argb([r, g, b, a]: [u8; 4]) -> u32 {
            (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
        }

        if let Some(state) = self.windows.get_mut(&window_id) {
            let size = state.window.inner_size();
            let old_width = state.pixmap.width();
            let old_height = state.pixmap.height();

            let resized = old_width != size.width || old_height != size.height;

            if resized {
                state.surface.resize(
                    NonZeroU32::new(size.width).unwrap(),
                    NonZeroU32::new(size.height).unwrap(),
                )?;

                state.pixmap = tiny_skia::Pixmap::new(size.width, size.height).unwrap();
            }

            let mut buffer = state.surface.buffer_mut()?;

            if let Some(draw) = self.app.draw_window(&mut self.data, window_id) {
                if state.clear_color != draw.clear_color || resized {
                    state.clear_color = draw.clear_color;

                    state.pixmap.fill(
                        tiny_skia::Color::from_rgba(
                            draw.clear_color.r,
                            draw.clear_color.g,
                            draw.clear_color.b,
                            draw.clear_color.a,
                        )
                        .unwrap(),
                    );
                }

                if resized {
                    render::render_canvas(&mut state.pixmap.as_mut(), draw.canvas, Vector::ZERO);
                } else {
                    let mut diff = draw.canvas.diff(&state.old_canvas);

                    let len = diff.rects().len();
                    diff.simplify();

                    tracing::trace!("Diff rects: {} -> {}", len, diff.rects().len());

                    for rect in diff.rects() {
                        if rect.size().min_element() < 1.0 {
                            continue;
                        }

                        // floor the points of the rect and add a 1 pixel border
                        let min = rect.min.floor() - 1.0;
                        let max = rect.max.ceil() + 1.0;

                        // convert the rect to integers
                        let x = min.x as i32;
                        let y = min.y as i32;
                        let w = (max.x - min.x).ceil() as u32;
                        let h = (max.y - min.y).ceil() as u32;

                        let mut pixmap = tiny_skia::Pixmap::new(w, h).unwrap();

                        pixmap.fill(
                            tiny_skia::Color::from_rgba(
                                draw.clear_color.r,
                                draw.clear_color.g,
                                draw.clear_color.b,
                                draw.clear_color.a,
                            )
                            .unwrap(),
                        );

                        render::render_canvas(
                            &mut pixmap.as_mut(),
                            draw.canvas,
                            Vector::new(x as f32, y as f32),
                        );

                        state.pixmap.draw_pixmap(
                            x,
                            y,
                            pixmap.as_ref(),
                            &tiny_skia::PixmapPaint {
                                blend_mode: tiny_skia::BlendMode::Source,
                                ..Default::default()
                            },
                            tiny_skia::Transform::identity(),
                            None,
                        );
                    }
                }

                state.old_canvas = draw.canvas.clone();
            }

            for (src, dst) in state.pixmap.pixels().iter().zip(buffer.iter_mut()) {
                let r = src.red();
                let g = src.green();
                let b = src.blue();
                let a = src.alpha();

                *dst = pack_argb([r, g, b, a]);
            }

            buffer.present()?;
        }

        Ok(())
    }

    fn scale_factor(&self, window_id: ori_core::window::WindowId) -> f32 {
        match self.windows.get(&window_id) {
            Some(state) => state.window.scale_factor() as f32,
            None => 1.0,
        }
    }

    fn window_event(&mut self, winit_id: winit::window::WindowId, event: WindowEvent) {
        // if the window id is not in the map, we ignore the event
        let Some(&id) = self.window_ids.get(&winit_id) else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {
                if let Err(err) = self.render(id) {
                    tracing::error!("Failed to render: {}", err);
                }
            }
            WindowEvent::CloseRequested => {
                self.app.close_requested(&mut self.data, id);
            }
            WindowEvent::Resized(inner_size) => {
                (self.app).window_resized(&mut self.data, id, inner_size.width, inner_size.height);
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                (self.app).window_scaled(&mut self.data, id, scale_factor as f32);
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => {
                let scale_factor = self.scale_factor(id);
                let position = Point::new(position.x as f32, position.y as f32) / scale_factor;
                self.app.pointer_moved(
                    &mut self.data,
                    id,
                    PointerId::from_hash(&device_id),
                    position,
                );
            }
            WindowEvent::CursorLeft { device_id } => {
                (self.app).pointer_left(&mut self.data, id, PointerId::from_hash(&device_id));
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
                ..
            } => {
                self.app.pointer_button(
                    &mut self.data,
                    id,
                    PointerId::from_hash(&device_id),
                    convert_mouse_button(button),
                    is_pressed(state),
                );
            }
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(x, y),
                device_id,
                ..
            } => self.app.pointer_scrolled(
                &mut self.data,
                id,
                PointerId::from_hash(&device_id),
                Vector::new(x, y),
            ),
            // since we're using a pointer model we need to handle touch
            // by emulating pointer events
            WindowEvent::Touch(event) => self.touch_event(id, event),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key,
                        text,
                        state,
                        ..
                    },
                ..
            } => {
                let code = match physical_key {
                    PhysicalKey::Code(code) => convert_key(code),
                    _ => None,
                };

                self.app.keyboard_key(
                    &mut self.data,
                    id,
                    code,
                    text.map(Into::into),
                    is_pressed(state),
                );
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.app.modifiers_changed(Modifiers {
                    shift: modifiers.state().contains(ModifiersState::SHIFT),
                    ctrl: modifiers.state().contains(ModifiersState::CONTROL),
                    alt: modifiers.state().contains(ModifiersState::ALT),
                    meta: modifiers.state().contains(ModifiersState::SUPER),
                });
            }
            _ => {}
        }
    }

    fn touch_event(&mut self, window_id: ori_core::window::WindowId, event: winit::event::Touch) {
        let scale_factor = self.scale_factor(window_id);
        let position = Point::new(event.location.x as f32, event.location.y as f32) / scale_factor;
        let pointer_id = PointerId::from_hash(&event.device_id);

        // we always send a pointer moved event first because the ui
        // needs to know where the pointer is. this will also ensure
        // that hot state is updated correctly
        (self.app).pointer_moved(&mut self.data, window_id, pointer_id, position);

        match event.phase {
            TouchPhase::Started => {
                self.app.pointer_button(
                    &mut self.data,
                    window_id,
                    pointer_id,
                    // a touch event is always the primary button
                    PointerButton::Primary,
                    true,
                );
            }
            TouchPhase::Moved => {}
            TouchPhase::Ended | TouchPhase::Cancelled => {
                self.app.pointer_button(
                    &mut self.data,
                    window_id,
                    pointer_id,
                    // a touch event is always the primary button
                    PointerButton::Primary,
                    false,
                );

                // we also need to send a pointer left event because
                // the ui needs to know that the pointer left the window
                self.app.pointer_left(&mut self.data, window_id, pointer_id);
            }
        }
    }
}
