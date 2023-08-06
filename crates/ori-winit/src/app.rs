use std::{
    env,
    error::Error,
    time::{Duration, Instant},
};

use ori_core::{
    math::Vec2, BoxedBuildUi, BuildUi, Modifiers, Ui, UiBuilder, Window, WindowBuilder,
};
use ori_reactive::Event;
use ori_style::Stylesheet;
use ori_wgpu::WgpuBackend;
use tracing::metadata::LevelFilter;
use tracing_subscriber::Layer;
use winit::{
    event::{Event as WinitEvent, KeyboardInput, MouseScrollDelta, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowId as WinitWindowId,
};

use crate::{
    backend::WinitBackend,
    convert::{convert_device_id, convert_key, convert_mouse_button, is_pressed},
};

fn init_tracing() -> Result<(), Box<dyn Error>> {
    use tracing_subscriber::layer::SubscriberExt;

    let mut filter = tracing_subscriber::EnvFilter::default()
        .add_directive("wgpu=warn".parse()?)
        .add_directive("naga=warn".parse()?)
        .add_directive("winit=warn".parse()?)
        .add_directive("mio=warn".parse()?)
        .add_directive("ori=warn".parse()?)
        .add_directive(LevelFilter::DEBUG.into());

    if let Ok(env) = env::var(tracing_subscriber::EnvFilter::DEFAULT_ENV) {
        filter = filter.add_directive(env.parse()?);
    }

    let subscriber = tracing_subscriber::registry();

    let fmt_layer = tracing_subscriber::fmt::Layer::default().with_filter(filter);
    let subscriber = subscriber.with(fmt_layer);

    #[cfg(feature = "tracy")]
    let subscriber = subscriber.with(tracing_tracy::TracyLayer::new());

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

/// A app using [`winit`] as the windowing backend.
pub struct App {
    window: Window,
    event_loop: EventLoop<(WinitWindowId, Event)>,
    ui: Ui<WinitBackend, WgpuBackend>,
    builder: Option<BoxedBuildUi>,
}

impl UiBuilder<WinitBackend, WgpuBackend> for App {
    fn ui(&mut self) -> &mut Ui<WinitBackend, WgpuBackend> {
        &mut self.ui
    }
}

impl WindowBuilder for App {
    fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }
}

impl App {
    /// Create a new [`App`] with the given content.
    pub fn new<I>(content: impl BuildUi<I>) -> Self {
        let event_loop = EventLoopBuilder::with_user_event().build();
        Self::new_with_event_loop(event_loop, content)
    }

    pub fn new_with_event_loop<I>(
        event_loop: EventLoop<(WinitWindowId, Event)>,
        content: impl BuildUi<I>,
    ) -> Self {
        init_tracing().unwrap();

        let window_backend = WinitBackend::new(event_loop.create_proxy());
        let wgpu_backend = WgpuBackend::new();
        let mut ui = Ui::new(window_backend, wgpu_backend);

        ui.style_loader.add_style(Stylesheet::day_theme()).unwrap();

        Self {
            window: Window::default(),
            ui,
            event_loop,
            builder: Some(content.boxed()),
        }
    }

    /// Set the default theme to night theme, this will clear all the styles
    /// that have been added before, and should therefore be called before
    /// [`App::style`].
    pub fn night_theme(mut self) -> Self {
        self.ui.style_loader.clear();
        self.ui.style_loader.add_style(Stylesheet::new()).unwrap();
        self.ui
            .style_loader
            .add_style(Stylesheet::night_theme())
            .unwrap();
        self
    }

    /// Set the default theme to day theme, this will clear all the styles
    /// that have been added before, and should therefore be called before
    /// [`App::style`].
    pub fn day_theme(mut self) -> Self {
        self.ui.style_loader.clear();
        self.ui.style_loader.add_style(Stylesheet::new()).unwrap();
        self.ui
            .style_loader
            .add_style(Stylesheet::day_theme())
            .unwrap();
        self
    }
}

impl App {
    pub fn new_any_thread<I>(content: impl BuildUi<I>) -> Self {
        let mut builder = EventLoopBuilder::with_user_event();

        #[cfg(target_os = "windows")]
        {
            use winit::platform::windows::EventLoopBuilderExtWindows;
            builder.with_any_thread(true);
        }

        #[cfg(all(target_os = "linux", feature = "x11"))]
        {
            use winit::platform::x11::EventLoopBuilderExtX11;
            builder.with_any_thread(true);
        }

        #[cfg(all(target_os = "linux", feature = "wayland"))]
        {
            use winit::platform::wayland::EventLoopBuilderExtWayland;
            builder.with_any_thread(true);
        }

        #[cfg(target_os = "macos")]
        {
            use winit::platform::macos::EventLoopBuilderExtMacOS;
            builder.with_any_thread(true);
        }

        Self::new_with_event_loop(builder.build(), content)
    }
}

impl App {
    /// Run the app.
    pub fn run(mut self) -> ! {
        self.ui.load_default_fonts();

        let root = self.builder.take().unwrap();
        (self.ui.create_window(&self.event_loop, &self.window, root)).unwrap();

        self.event_loop.run(move |event, target, control_flow| {
            *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(10));

            match event {
                WinitEvent::RedrawRequested(window) => {
                    #[cfg(feature = "tracy")]
                    tracing_tracy::client::frame_mark();

                    if let Some(id) = self.ui.window_backend.id(window) {
                        self.ui.draw(id);
                    }
                }
                WinitEvent::MainEventsCleared
                | WinitEvent::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                    self.ui.idle();
                }
                WinitEvent::UserEvent((window, event)) => {
                    let Some(id) = self.ui.window_backend.id(window) else {
                        return;
                    };

                    self.ui.event(target, id, &event);

                    if self.ui.is_empty() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                WinitEvent::WindowEvent {
                    event,
                    window_id: window,
                    ..
                } => {
                    let Some(window) = self.ui.window_backend.id(window) else {
                        return;
                    };

                    match event {
                        WindowEvent::Resized(size)
                        | WindowEvent::ScaleFactorChanged {
                            new_inner_size: &mut size,
                            ..
                        } => {
                            self.ui.window_resized(window, size.width, size.height);
                        }
                        WindowEvent::CloseRequested => {
                            self.ui.close_window(window);

                            if self.ui.is_empty() {
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                        WindowEvent::CursorMoved {
                            position,
                            device_id,
                            ..
                        } => {
                            let device = convert_device_id(device_id);
                            let position = Vec2::new(position.x as f32, position.y as f32);
                            self.ui.pointer_moved(window, device, position);
                        }
                        WindowEvent::CursorLeft { device_id } => {
                            let device = convert_device_id(device_id);
                            self.ui.pointer_left(window, device);
                        }
                        WindowEvent::MouseInput {
                            button,
                            state: element_state,
                            device_id,
                            ..
                        } => {
                            let device = convert_device_id(device_id);
                            let button = convert_mouse_button(button);
                            let pressed = is_pressed(element_state);
                            self.ui.pointer_button(window, device, button, pressed);
                        }
                        WindowEvent::MouseWheel {
                            delta: MouseScrollDelta::LineDelta(x, y),
                            device_id,
                            ..
                        } => {
                            let device = convert_device_id(device_id);
                            let delta = Vec2::new(x, y);
                            self.ui.pointer_scroll(window, device, delta);
                        }
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(virtual_keycode),
                                    state: element_state,
                                    ..
                                },
                            ..
                        } => {
                            let key = convert_key(virtual_keycode);
                            let pressed = is_pressed(element_state);

                            if let Some(key) = key {
                                self.ui.key(window, key, pressed);
                            }
                        }
                        WindowEvent::ReceivedCharacter(c) => {
                            self.ui.text(window, String::from(c));
                        }
                        WindowEvent::ModifiersChanged(new_modifiers) => {
                            let modifiers = Modifiers {
                                shift: new_modifiers.shift(),
                                ctrl: new_modifiers.ctrl(),
                                alt: new_modifiers.alt(),
                                meta: new_modifiers.logo(),
                            };

                            self.ui.modifiers_changed(window, modifiers);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    }
}
