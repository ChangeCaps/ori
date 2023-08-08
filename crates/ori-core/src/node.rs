use std::{fmt::Debug, sync::Arc};

use glam::Vec2;
use ori_graphics::{Affine, Fonts, Frame, ImageCache, Renderer};
use ori_reactive::{Event, EventSink};

use crate::{
    AvailableSpace, Context, DrawContext, EventContext, IntoView, LayoutContext, Tree, View,
};

#[derive(Clone)]
pub struct Node {
    view: Arc<dyn View>,
}

impl Default for Node {
    fn default() -> Self {
        Self::empty()
    }
}

impl Node {
    pub fn new(view: impl IntoView) -> Self {
        view.into_node()
    }

    pub fn from_arc(view: Arc<dyn View>) -> Self {
        Self { view }
    }

    pub fn empty() -> Self {
        Self { view: Arc::new(()) }
    }

    pub(crate) fn event_root(
        &self,
        fonts: &mut Fonts,
        renderer: &dyn Renderer,
        image_cache: &mut ImageCache,
        event_sink: &EventSink,
        tree: &mut Tree,
        event: &Event,
    ) {
        let context = Context::new(fonts, renderer, image_cache, event_sink, tree);
        let mut cx = EventContext::new(context, Affine::IDENTITY);
        self.view.event(&mut cx, event);
    }

    pub(crate) fn layout_root(
        &self,
        fonts: &mut Fonts,
        renderer: &dyn Renderer,
        image_cache: &mut ImageCache,
        event_sink: &EventSink,
        tree: &mut Tree,
        space: AvailableSpace,
    ) -> Vec2 {
        let context = Context::new(fonts, renderer, image_cache, event_sink, tree);
        let mut cx = LayoutContext::new(context);
        let size = self.view.layout(&mut cx, space);
        tree.size = Some(size);
        size
    }

    pub(crate) fn draw_root(
        &self,
        fonts: &mut Fonts,
        renderer: &dyn Renderer,
        image_cache: &mut ImageCache,
        event_sink: &EventSink,
        tree: &mut Tree,
        frame: &mut Frame,
    ) {
        let context = Context::new(fonts, renderer, image_cache, event_sink, tree);
        let mut cx = DrawContext::new(context, frame);
        self.view.draw(&mut cx);
    }
}

impl View for Node {
    fn event(&self, cx: &mut EventContext<'_>, event: &Event) {
        self.view.event(cx, event);
    }

    fn layout(&self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        self.view.layout(cx, space)
    }

    fn draw(&self, cx: &mut DrawContext<'_>) {
        self.view.draw(cx);
    }

    fn into_node(self) -> Node {
        self
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node").finish()
    }
}
