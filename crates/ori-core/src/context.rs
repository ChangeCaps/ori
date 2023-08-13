use std::{any::Any, future::Future, time::Duration};

use ori_graphics::{math::Vec2, Fonts, ImageCache, Renderer};
use ori_reactive::{EventSink, EventTask, Signal};

use crate::{RequestRedrawEvent, Theme, Tree, Unit, Window};

pub struct Context<'a> {
    pub fonts: &'a mut Fonts,
    pub renderer: &'a dyn Renderer,
    pub image_cache: &'a mut ImageCache,
    pub theme: &'a Theme,
    pub window: Signal<Window>,
    pub delta_time: Duration,
    pub(crate) event_sink: &'a EventSink,
    pub(crate) tree: &'a mut Tree,
}

impl<'a> Context<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        fonts: &'a mut Fonts,
        renderer: &'a dyn Renderer,
        image_cache: &'a mut ImageCache,
        theme: &'a Theme,
        window: Signal<Window>,
        delta_time: Duration,
        event_sink: &'a EventSink,
        tree: &'a mut Tree,
    ) -> Self {
        Self {
            fonts,
            renderer,
            image_cache,
            theme,
            window,
            delta_time,
            event_sink,
            tree,
        }
    }

    pub(crate) fn child<T>(&mut self, index: usize, f: impl FnOnce(Context<'_>) -> T) -> T {
        let mut context = self.borrow();
        context.tree = context.tree.child(index);

        f(context)
    }

    pub(crate) fn size(&self) -> Vec2 {
        match self.tree.size() {
            Some(size) => size,
            None => {
                tracing::error!("Node hasn't been laid out");
                Vec2::ZERO
            }
        }
    }

    pub fn borrow(&mut self) -> Context<'_> {
        Context {
            fonts: self.fonts,
            renderer: self.renderer,
            image_cache: self.image_cache,
            theme: self.theme,
            window: self.window,
            delta_time: self.delta_time,
            event_sink: self.event_sink,
            tree: self.tree,
        }
    }

    /// Get the event sink for this context.
    pub fn event_sink(&self) -> &EventSink {
        self.event_sink
    }

    /// Spawn a future on the event loop.
    pub fn spawn_future(&self, future: impl Future<Output = ()> + Send + 'static) {
        EventTask::spawn(self.event_sink.clone(), future);
    }

    pub fn unit(&self, unit: Unit) -> f32 {
        let window = self.window.get();
        unit.resolve(window.scale, window.size.as_vec2())
    }

    pub fn emit(&self, event: impl Any + Send + Sync) {
        self.event_sink.send(event);
    }

    pub fn request_redraw(&self) {
        self.emit(RequestRedrawEvent);
    }

    pub fn dt(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }
}
