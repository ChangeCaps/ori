use std::{any::Any, future::Future, time::Duration};

use ori_graphics::{math::Vec2, Fonts, ImageCache, Renderer};
use ori_reactive::{EventSink, EventTask};

use crate::{Cursor, RequestLayoutEvent, RequestRedrawEvent, Theme, Unit, ViewState, Window};

pub struct Context<'a> {
    pub fonts: &'a mut Fonts,
    pub renderer: &'a dyn Renderer,
    pub image_cache: &'a mut ImageCache,
    pub theme: &'a Theme,
    pub window: &'a Window,
    pub delta_time: Duration,
    pub cursor: &'a mut Cursor,
    pub(crate) view_state: &'a mut ViewState,
    pub(crate) event_sink: &'a EventSink,
}

impl<'a> Context<'a> {
    pub(crate) fn size(&self) -> Vec2 {
        self.view_state.size
    }

    /// Set whether the view is active.
    pub fn set_active(&mut self, active: bool) {
        self.view_state.set_active(active);
    }

    /// Set whether the view is hovered.
    pub fn set_hovered(&mut self, hovered: bool) {
        self.view_state.set_hovered(hovered);
    }

    /// Get whether the view is active.
    pub fn is_active(&self) -> bool {
        self.view_state.active
    }

    /// Get whether the view is hovered.
    pub fn is_hovered(&self) -> bool {
        self.view_state.hovered
    }

    pub fn borrow(&mut self) -> Context<'_> {
        Context {
            fonts: self.fonts,
            renderer: self.renderer,
            image_cache: self.image_cache,
            theme: self.theme,
            window: self.window,
            delta_time: self.delta_time,
            cursor: self.cursor,
            view_state: self.view_state,
            event_sink: self.event_sink,
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

    pub fn set_cursor(&mut self, cursor: Cursor) {
        *self.cursor = cursor;
    }

    pub fn unit(&self, unit: Unit) -> f32 {
        unit.resolve(self.window.scale, self.window.size.as_vec2())
    }

    pub fn emit(&self, event: impl Any + Send + Sync) {
        self.event_sink.send(event);
    }

    pub fn request_redraw(&self) {
        self.emit(RequestRedrawEvent);
    }

    pub fn request_layout(&self) {
        self.emit(RequestLayoutEvent);
    }

    pub fn dt(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }
}
