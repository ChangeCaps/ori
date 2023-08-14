use std::{
    collections::HashMap,
    fmt::Debug,
    time::{Duration, Instant},
};

use ori_graphics::{
    math::{UVec2, Vec2},
    FontSource, Fonts, Frame, ImageCache, RenderBackend, Renderer,
};
use ori_reactive::{Emitter, Event, EventSink, EventTask, Scope, WeakCallback};

use crate::{
    function::{react, window},
    AvailableSpace, CloseWindow, Code, Context, Cursor, DragWindow, KeyboardEvent, Metrics,
    Modifiers, Node, OpenWindow, Palette, PointerButton, PointerEvent, RequestAnimationFrame,
    RequestLayoutEvent, RequestRedrawEvent, Theme, Window, WindowBackend, WindowClosedEvent,
    WindowId, WindowResizedEvent,
};

const TEXT_FONT: &[u8] = include_bytes!("../fonts/NotoSans-Medium.ttf");

/// A trait for building user interfaces.
pub trait BuildUi<V>: Sized + Send + Sync + 'static {
    /// Build the user interface.
    fn build(&mut self, cx: Scope) -> Node;

    /// Convert this builder into a function.
    fn function(mut self) -> UiFunction {
        Box::new(move |scope| self.build(scope))
    }
}

impl<V: Into<Node>, F> BuildUi<V> for F
where
    F: FnMut(Scope) -> V + Send + Sync + 'static,
{
    fn build(&mut self, scope: Scope) -> Node {
        Node::new(self(scope))
    }
}

pub type UiFunction = Box<dyn FnMut(Scope) -> Node + Send + Sync + 'static>;

struct WindowUi<R: Renderer> {
    renderer: R,
    window: Window,
    root: Node,
    scope: Scope,
    event_sink: EventSink,
    event_emitter: Emitter<Event>,
    modifiers: Modifiers,
    pointers: HashMap<u64, Vec2>,
    animation_frames: Vec<WeakCallback>,
    needs_layout: bool,
    last_event: Option<Instant>,
    last_layout: Option<Instant>,
    last_draw: Option<Instant>,
}

impl<R: Renderer> WindowUi<R> {
    fn event_delta(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now - self.last_event.unwrap_or(now);
        self.last_event = Some(now);
        delta
    }

    fn layout_delta(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now - self.last_layout.unwrap_or(now);
        self.last_layout = Some(now);
        delta
    }

    fn draw_delta(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now - self.last_draw.unwrap_or(now);
        self.last_draw = Some(now);
        delta
    }

    /// Queries information about the window that won't be provided by events.
    fn query_window(&mut self, window_backend: &mut impl WindowBackend) {
        let mut new_window = window(self.scope).get_untracked();

        new_window.minimized = window_backend.get_minimized(self.window.id());
        new_window.maximized = window_backend.get_maximized(self.window.id());
        new_window.size = window_backend.get_size(self.window.id());

        if new_window != window(self.scope).get_untracked() {
            self.window.minimized = new_window.minimized;
            self.window.maximized = new_window.maximized;
            self.window.size = new_window.size;

            window(self.scope).set(new_window);
        }
    }

    fn update_cursor(&self, cursor: Cursor) {
        let window = window(self.scope);
        let window_cursor = window.get_untracked().cursor;

        let window_set = window_cursor != self.window.cursor;

        if window_cursor != cursor && !window_set {
            window.modify().cursor = cursor;
        }
    }

    fn update_window(&mut self, window_backend: &mut impl WindowBackend, window: &Window) {
        if self.window.title != window.title {
            self.window.title = window.title.clone();
            window_backend.set_title(window.id(), window.title.clone());
            tracing::debug!("Window {} title set to '{}'", window.id(), window.title);
        }

        if self.window.resizable != window.resizable {
            self.window.resizable = window.resizable;
            window_backend.set_resizable(window.id(), window.resizable);
            tracing::debug!(
                "Window {} resizable set to '{}'",
                window.id(),
                window.resizable
            );
        }

        if self.window.decorated != window.decorated {
            self.window.decorated = window.decorated;
            window_backend.set_decorations(window.id(), window.decorated);
            tracing::debug!(
                "Window {} decorations set to '{}'",
                window.id(),
                window.decorated
            );
        }

        if self.window.clear_color != window.clear_color {
            self.window.clear_color = window.clear_color;
            window_backend.set_transparent(window.id(), window.clear_color.is_translucent());
            tracing::debug!(
                "Window {} clear color set to {}",
                window.id(),
                window.clear_color.to_hex(),
            );
        }

        if self.window.icon != window.icon {
            self.window.icon = window.icon.clone();
            window_backend.set_icon(window.id(), window.icon.clone());
            tracing::debug!("Window {} icon set", window.id());
        }

        if self.window.size != window.size {
            self.window.size = window.size;
            window_backend.set_size(window.id(), window.size);
            tracing::debug!("Window {} size set to {}", window.id(), window.size);
        }

        if self.window.minimized != window.minimized {
            self.window.minimized = window.minimized;
            window_backend.set_minimized(window.id(), window.minimized);
            tracing::debug!(
                "Window {} minimized set to {}",
                window.id(),
                window.minimized
            );
        }

        if self.window.maximized != window.maximized {
            self.window.maximized = window.maximized;
            window_backend.set_maximized(window.id(), window.maximized);
            tracing::debug!(
                "Window {} maximized set to {}",
                window.id(),
                window.maximized
            );
        }

        if self.window.visible != window.visible {
            self.window.visible = window.visible;
            window_backend.set_visible(window.id(), window.visible);
            tracing::debug!(
                "Window {} visibility set to {}",
                window.id(),
                window.visible
            );
        }

        if self.window.cursor != window.cursor {
            self.window.cursor = window.cursor;
            window_backend.set_cursor(window.id(), window.cursor);
            tracing::trace!("Window {} cursor set to {:?}", window.id(), window.cursor);
        }
    }
}

/// An error that can occur when creating a window.
pub enum WindowError<W: WindowBackend, R: RenderBackend> {
    WindowBackend(W::Error),
    RenderBackend(R::Error),
}

impl<W: WindowBackend, R: RenderBackend> Debug for WindowError<W, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowError::WindowBackend(err) => write!(f, "WindowBackend({:?})", err),
            WindowError::RenderBackend(err) => write!(f, "RenderBackend({:?})", err),
        }
    }
}

/// A callback that is called when the application is idle.
pub type IdleCallback<W, R> = dyn FnMut(&mut Ui<W, R>) + 'static;

/// Configuration for the [`Ui`] system.
#[derive(Clone, Debug)]
pub struct UiConfig {
    /// When enabled, pointer moved events will only be emitted just before drawing.
    ///
    /// This is useful because on some platforms, pointer moved events are emitted
    /// at a very high rate, (up to 1000 times per second), which can cause a lot of
    /// unnecessary work to be done.
    pub reduce_pointer_events: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for UiConfig {
    fn default() -> Self {
        Self {
            reduce_pointer_events: false,
        }
    }
}

/// The main entry point for the UI system.
///
/// When implementing a custom shell, this is primarily the type that you will interact with.
pub struct Ui<W, R>
where
    W: WindowBackend,
    R: RenderBackend<Surface = W::Surface>,
{
    /// The window backend, see [`WindowBackend`] for more information.
    pub window_backend: W,
    /// The render backend, see [`RenderBackend`] for more information.
    pub render_backend: R,
    /// The current frame, this is only stored here to avoid allocations.
    pub frame: Frame,
    /// The font system, see [`Fonts`] for more information.
    pub fonts: Fonts,
    /// The theme, see [`Theme`] for more information.
    pub theme: Theme,
    /// The image cache, see [`ImageCache`] for more information.
    pub image_cache: ImageCache,
    /// The idle callback, see [`Ui::idle`] for more information.
    pub idle_callback: Option<Box<IdleCallback<W, R>>>,
    /// The configuration, see [`UiConfig`] for more information.
    pub config: UiConfig,
    /// The metrics, see [`Metrics`] for more information.
    pub metrics: Metrics,

    needs_pointer_moved: Vec<WindowId>,
    window_ui: HashMap<WindowId, WindowUi<R::Renderer>>,
}

impl<W, R> Ui<W, R>
where
    W: WindowBackend,
    R: RenderBackend<Surface = W::Surface>,
{
    /// Creates a new [`Ui`] instance.
    ///
    /// **Note** that `W` and `R` need to have the same `Surface` type.
    pub fn new(window_backend: W, render_backend: R) -> Self {
        let fonts = Fonts::new();

        let mut theme = Theme::builtin();
        theme.extend(Palette::light().into());

        Self {
            window_backend,
            render_backend,
            frame: Frame::new(),
            fonts,
            theme,
            image_cache: ImageCache::new(),
            idle_callback: None,
            config: UiConfig::default(),
            metrics: Metrics::new(),
            needs_pointer_moved: Vec::new(),
            window_ui: HashMap::new(),
        }
    }

    /// Loads the default fonts.
    pub fn load_default_fonts(&mut self) {
        self.fonts.load_system_fonts();
        self.fonts.load_font_data(TEXT_FONT.to_vec());
    }

    /// Returns the number of windows.
    pub fn len(&self) -> usize {
        self.window_ui.len()
    }

    /// Returns `true` if there are no windows.
    pub fn is_empty(&self) -> bool {
        self.window_ui.is_empty()
    }

    /// Returns the [`WindowId`]s of all windows.
    pub fn window_ids(&self) -> Vec<WindowId> {
        self.window_ui.keys().copied().collect()
    }

    /// Creates a new window.
    pub fn create_window(
        &mut self,
        target: W::Target<'_>,
        window: &Window,
        ui: UiFunction,
    ) -> Result<(), WindowError<W, R>> {
        {
            // here we clone the window and set it to invisible
            // the visibility will be set once the window and renderer is created
            // this is to ensure that the window is only show when rendering is ready
            //
            // see https://docs.rs/winit/0.28.6/winit/#drawing-on-the-window
            let mut window = window.clone();
            window.visible = false;

            self.window_backend
                .create_window(target, &window)
                .map_err(WindowError::WindowBackend)?;
        }

        let surface = self
            .window_backend
            .create_surface(window.id())
            .map_err(WindowError::WindowBackend)?;

        let renderer = self
            .render_backend
            .create_renderer(surface, window.size.x, window.size.y)
            .map_err(WindowError::RenderBackend)?;

        let event_sink = self
            .window_backend
            .create_event_sink(window.id())
            .map_err(WindowError::WindowBackend)?;

        let event_emitter = Emitter::new();
        let scope = Scope::new(event_sink.clone(), event_emitter.clone());
        scope.with_context(scope.signal(window.clone()));

        // create the root view
        let root = Node::new(react(scope, ui));

        let window_ui = WindowUi {
            renderer,
            window: window.clone(),
            root,
            scope,
            event_sink,
            event_emitter,
            modifiers: Modifiers::default(),
            pointers: HashMap::new(),
            animation_frames: Vec::new(),
            needs_layout: true,
            last_event: None,
            last_layout: None,
            last_draw: None,
        };

        // now that everything is ready, we can show the window
        self.window_backend.set_visible(window.id(), window.visible);
        self.window_ui.insert(window.id(), window_ui);

        tracing::debug!("Window {} created", window.id());

        Ok(())
    }

    /// Close a window.
    pub fn close_window(&mut self, id: WindowId) {
        self.window_backend.close_window(id);

        if let Some(ui) = self.window_ui.remove(&id) {
            ui.scope.dispose();
        }

        tracing::debug!("Window {} closed", id);

        for window in self.window_ids() {
            self.event_inner(window, &Event::new(WindowClosedEvent::new(id)), true);
        }
    }

    /// Run when the application is idle.
    ///
    /// This will reload styles if necessary, among other things.
    pub fn idle(&mut self) {
        self.metrics.log();

        self.image_cache.clean();

        if let Some(mut idle) = self.idle_callback.take() {
            idle(self);
            self.idle_callback = Some(idle);
        }
    }

    /// Forces all elements to be redrawn.
    pub fn request_redraw(&mut self) {
        for id in self.window_ids() {
            self.window_backend.request_redraw(id);
        }
    }

    /// Forces all elements to be relaid out.
    pub fn request_layout(&mut self) {
        for (&id, ui) in self.window_ui.iter_mut() {
            ui.needs_layout = true;
            self.window_backend.request_redraw(id);
        }
    }

    /// Window has been resized.
    pub fn window_resized(&mut self, id: WindowId, width: u32, height: u32) {
        if let Some(ui) = self.window_ui.get_mut(&id) {
            ui.needs_layout = true;

            window(ui.scope).modify().size = UVec2::new(width, height);
            ui.renderer.resize(width, height);
        }

        let event = WindowResizedEvent::new(Vec2::new(width as f32, height as f32));
        self.event_inner(id, &Event::new(event), true);
    }

    /// Get the position of a pointer with a given `device` with a given `id`.
    pub fn get_pointer_position(&mut self, window: WindowId, device: u64) -> Option<Vec2> {
        let window = self.window_ui.get_mut(&window)?;
        Some(*window.pointers.entry(device).or_default())
    }

    /// Get the position of a pointer with a given `device` with a given `id`.
    ///
    /// If the pointer is not found, (0, 0) is returned.
    pub fn pointer_position(&mut self, window: WindowId, device: u64) -> Vec2 {
        (self.get_pointer_position(window, device)).unwrap_or_default()
    }

    /// Get the modifiers of a window.
    pub fn get_modfiers(&self, window: WindowId) -> Option<Modifiers> {
        Some(self.window_ui.get(&window)?.modifiers)
    }

    pub fn needs_layout(&self, window: WindowId) -> bool {
        (self.window_ui.get(&window)).map_or(false, |ui| ui.needs_layout)
    }

    /// Get the modifiers of a window.
    ///
    /// If the window is not found, [`Modifiers::default()`] is returned.
    pub fn modifiers(&self, window: WindowId) -> Modifiers {
        self.get_modfiers(window).unwrap_or_default()
    }

    fn generate_pointer_moved(&self, window: WindowId) -> Vec<PointerEvent> {
        match self.window_ui.get(&window) {
            Some(ui) => {
                let mut events = Vec::new();

                for (&device, &position) in ui.pointers.iter() {
                    events.push(PointerEvent {
                        device,
                        position,
                        modifiers: ui.modifiers,
                        ..Default::default()
                    });
                }

                events
            }
            None => Vec::new(),
        }
    }

    fn simulate_pointer_moved(&mut self, window: WindowId) {
        if let Some(i) = self.needs_pointer_moved.iter().position(|&i| i == window) {
            self.needs_pointer_moved.swap_remove(i);

            for pointer_event in self.generate_pointer_moved(window) {
                self.metrics.pointer_moved.event();

                let event = Event::new(pointer_event);
                self.event_inner(window, &event, false);
            }
        }
    }

    fn emit_animation_frames(&mut self, window: WindowId) {
        if let Some(ui) = self.window_ui.get_mut(&window) {
            for callback in ui.animation_frames.drain(..) {
                callback.emit(&());
            }
        }
    }

    /// Handle a pointer being moved.
    pub fn pointer_moved(&mut self, window: WindowId, device: u64, position: Vec2) {
        if let Some(window) = self.window_ui.get_mut(&window) {
            window.pointers.insert(device, position);
        }

        // if we are reducing pointer events, we won't send the event now
        // instead, we will send it when the window is redrawn
        if self.config.reduce_pointer_events {
            if !self.needs_pointer_moved.contains(&window) {
                self.needs_pointer_moved.push(window);
                self.window_backend.request_redraw(window);
            }

            return;
        }

        let event = PointerEvent {
            device,
            position,
            modifiers: self.modifiers(window),
            ..Default::default()
        };

        self.metrics.pointer_moved.event();
        self.event_inner(window, &Event::new(event), true);
    }

    /// Handle a pointer leaving the window.
    pub fn pointer_left(&mut self, window: WindowId, device: u64) {
        let event = PointerEvent {
            device,
            position: self.pointer_position(window, device),
            modifiers: self.modifiers(window),
            left: true,
            ..Default::default()
        };

        if let Some(window) = self.window_ui.get_mut(&window) {
            window.pointers.remove(&device);
        }

        self.event_inner(window, &Event::new(event), true);
    }

    /// Handle a pointer button being pressed or released.
    pub fn pointer_button(
        &mut self,
        window: WindowId,
        device: u64,
        button: PointerButton,
        pressed: bool,
    ) {
        let event = PointerEvent {
            device,
            position: self.pointer_position(window, device),
            modifiers: self.modifiers(window),
            button: Some(button),
            pressed,
            ..Default::default()
        };

        self.event_inner(window, &Event::new(event), true);
    }

    /// Handle a pointer being scrolled.
    pub fn pointer_scroll(&mut self, window: WindowId, device: u64, delta: Vec2) {
        let event = PointerEvent {
            device,
            position: self.pointer_position(window, device),
            modifiers: self.modifiers(window),
            scroll_delta: delta,
            ..Default::default()
        };

        self.event_inner(window, &Event::new(event), true);
    }

    /// Handle a key being pressed or released.
    pub fn key(&mut self, window: WindowId, key: Code, pressed: bool) {
        let event = KeyboardEvent {
            key: Some(key),
            pressed,
            modifiers: self.modifiers(window),
            ..Default::default()
        };

        self.event_inner(window, &Event::new(event), true);
    }

    /// Handle text being input.
    pub fn text(&mut self, window: WindowId, text: String) {
        let event = KeyboardEvent {
            text: Some(text),
            modifiers: self.modifiers(window),
            ..Default::default()
        };

        self.event_inner(window, &Event::new(event), true);
    }

    /// Handle modifiers being changed.
    pub fn modifiers_changed(&mut self, window: WindowId, modifiers: Modifiers) {
        if let Some(window) = self.window_ui.get_mut(&window) {
            window.modifiers = modifiers;
        }

        let event = KeyboardEvent {
            modifiers,
            ..Default::default()
        };

        self.event_inner(window, &Event::new(event), true);
    }

    /// Handle an [`Event`].
    ///
    /// This should be called every time an [`Event`] is received from the [`EventSink`].
    pub fn event(&mut self, target: W::Target<'_>, id: WindowId, event: &Event) {
        if let Some(task) = event.get::<EventTask>() {
            task.poll();
            return;
        }

        if let Some(event) = event.get::<CloseWindow>() {
            match event.window {
                Some(id) => self.close_window(id),
                None => self.close_window(id),
            }

            return;
        }

        if let Some(event) = event.get::<OpenWindow>() {
            if let Err(err) = self.create_window(target, event.window(), event.take_ui()) {
                tracing::error!("Failed to create window {}: {:?}", event.window().id(), err);
            }

            return;
        }

        if let Some(event) = event.get::<DragWindow>() {
            match event.window {
                Some(id) => self.window_backend.drag_window(id),
                None => self.window_backend.drag_window(id),
            }

            return;
        }

        if let Some(event) = event.get::<RequestAnimationFrame>() {
            if let Some(ui) = self.window_ui.get_mut(&id) {
                ui.animation_frames.push(event.callback().clone());
            }
        }

        if event.is::<RequestRedrawEvent>() {
            self.window_backend.request_redraw(id);
            return;
        }

        if event.is::<RequestLayoutEvent>() {
            if let Some(ui) = self.window_ui.get_mut(&id) {
                ui.needs_layout = true;
            }

            self.window_backend.request_redraw(id);
            return;
        }

        for id in self.window_ids() {
            self.event_inner(id, event, true);
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn event_inner(&mut self, id: WindowId, event: &Event, update_window: bool) {
        tracing::trace!("Event for window {:?}: {:?}", id, event.type_name());

        let Some(ui) = self.window_ui.get_mut(&id) else { return };
        ui.event_emitter.emit(event);
        ui.query_window(&mut self.window_backend);

        let window = window(ui.scope);
        let mut cursor = window.get().cursor;

        let delta_time = ui.event_delta();
        let start = Instant::now();
        ori_reactive::effect::delay_effects(|| {
            let context = Context {
                fonts: &mut self.fonts,
                renderer: &ui.renderer,
                image_cache: &mut self.image_cache,
                theme: &self.theme,
                window: &window.get(),
                delta_time,
                cursor: &mut cursor,
                view_state: &mut Default::default(),
                event_sink: &ui.event_sink,
            };

            ui.root.event_root(context, event);
        });
        self.metrics.event.event(start.elapsed());

        ui.update_cursor(cursor);

        if update_window {
            ui.update_window(&mut self.window_backend, &window.get());
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn layout_inner(&mut self, id: WindowId, update_window: bool) {
        tracing::trace!("Laying out window {:?}", id);

        if let Some(ui) = self.window_ui.get_mut(&id) {
            ui.needs_layout = false;

            ui.query_window(&mut self.window_backend);
            let window = window(ui.scope);
            let size = window.get_untracked().size.as_vec2();
            let mut cursor = window.get().cursor;

            let delta_time = ui.layout_delta();
            let start = Instant::now();
            ori_reactive::effect::delay_effects(|| {
                let context = Context {
                    fonts: &mut self.fonts,
                    renderer: &ui.renderer,
                    image_cache: &mut self.image_cache,
                    theme: &self.theme,
                    window: &window.get(),
                    delta_time,
                    cursor: &mut cursor,
                    view_state: &mut Default::default(),
                    event_sink: &ui.event_sink,
                };

                let space = AvailableSpace::new(Vec2::ZERO, size);
                ui.root.layout_root(context, space);
            });
            self.metrics.layout.event(start.elapsed());

            ui.update_cursor(cursor);

            if update_window {
                ui.update_window(&mut self.window_backend, &window.get());
            }
        }
    }

    /// Layout a window.
    pub fn layout(&mut self, id: WindowId) {
        self.layout_inner(id, true);
    }

    /// Draw a window.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn draw(&mut self, id: WindowId) {
        // if reducing pointer events, simulate them now
        self.simulate_pointer_moved(id);
        self.emit_animation_frames(id);

        tracing::trace!("Drawing window {:?}", id);

        // layout if needed
        if self.needs_layout(id) {
            self.layout_inner(id, false);
        }

        if let Some(ui) = self.window_ui.get_mut(&id) {
            self.frame.clear();

            let window = window(ui.scope);
            let mut cursor = window.get().cursor;

            let delta_time = ui.draw_delta();
            let start = Instant::now();
            ori_reactive::effect::delay_effects(|| {
                let context = Context {
                    fonts: &mut self.fonts,
                    renderer: &ui.renderer,
                    image_cache: &mut self.image_cache,
                    theme: &self.theme,
                    window: &window.get(),
                    delta_time,
                    cursor: &mut cursor,
                    view_state: &mut Default::default(),
                    event_sink: &ui.event_sink,
                };

                ui.root.draw_root(context, &mut self.frame);
            });
            self.metrics.draw.event(start.elapsed());

            ui.update_cursor(cursor);
            ui.update_window(&mut self.window_backend, &window.get());

            let clear_color = window.get().clear_color;
            (ui.renderer).render_frame(&self.frame, clear_color);
        }
    }
}

pub trait UiBuilder<W, R>: Sized
where
    W: WindowBackend,
    R: RenderBackend<Surface = <W as WindowBackend>::Surface>,
{
    /// Get a mutable reference to the [`Ui`] instance.
    fn ui(&mut self) -> &mut Ui<W, R>;

    /// Extend the theme, see [`Theme`].
    fn theme(mut self, theme: Theme) -> Self {
        self.ui().theme.extend(theme);
        self
    }

    /// Set the color palette, see [`Palette`].
    fn palette(self, palette: Palette) -> Self {
        self.theme(palette.into())
    }

    /// Loads a font from `source`, see [`font`](ori_graphics::font).
    fn font(mut self, font: impl Into<FontSource>) -> Self {
        if let Err(err) = self.ui().fonts.load_font(font) {
            tracing::error!("failed to load font: {:?}", err);
        }

        self
    }

    /// Set the idle callback, this is called when the app is idle.
    ///
    /// See [`Ui::idle`] for more information.
    fn idle(mut self, idle: impl FnMut(&mut Ui<W, R>) + 'static) -> Self {
        self.ui().idle_callback = Some(Box::new(idle));
        self
    }
}
