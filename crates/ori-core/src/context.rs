use std::any::Any;

use glam::Vec2;
use ori_graphics::{Fonts, ImageCache, Renderer};
use ori_reactive::EventSink;

use crate::{RequestRedrawEvent, Tree};

pub struct Context<'a> {
    pub fonts: &'a mut Fonts,
    pub renderer: &'a dyn Renderer,
    pub image_cache: &'a mut ImageCache,
    pub(crate) event_sink: &'a EventSink,
    pub(crate) tree: &'a mut Tree,
}

impl<'a> Context<'a> {
    pub(crate) fn new(
        fonts: &'a mut Fonts,
        renderer: &'a dyn Renderer,
        image_cache: &'a mut ImageCache,
        event_sink: &'a EventSink,
        tree: &'a mut Tree,
    ) -> Self {
        Self {
            fonts,
            renderer,
            image_cache,
            event_sink,
            tree,
        }
    }

    pub(crate) fn child<T>(&mut self, index: usize, f: impl FnOnce(Context<'_>) -> T) -> T {
        let context = Context {
            fonts: self.fonts,
            renderer: self.renderer,
            image_cache: self.image_cache,
            event_sink: self.event_sink,
            tree: self.tree.child(index),
        };

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

    pub fn emit(&self, event: impl Any + Send + Sync) {
        self.event_sink.emit(event);
    }

    pub fn request_redraw(&self) {
        self.emit(RequestRedrawEvent);
    }
}
