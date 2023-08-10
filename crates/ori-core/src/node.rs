use std::{fmt::Debug, sync::Arc};

use glam::Vec2;
use ori_reactive::Event;

use crate::{AvailableSpace, DrawContext, EventContext, LayoutContext, Padding, View};

impl<T: View> From<T> for Node {
    fn from(view: T) -> Self {
        Self {
            view: Arc::new(view),
        }
    }
}

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
    pub fn new(view: impl Into<Node>) -> Self {
        view.into()
    }

    pub fn from_arc(view: Arc<dyn View>) -> Self {
        Self { view }
    }

    pub fn empty() -> Self {
        Self { view: Arc::new(()) }
    }

    pub fn view(&self) -> &dyn View {
        self.view.as_ref()
    }

    pub fn downcast_ref<T: View>(&self) -> Option<&T> {
        self.view.downcast_ref()
    }
}

impl Node {
    pub fn event_indexed(&self, index: usize, cx: &mut EventContext<'_>, event: &Event) {
        cx.context.child(index, |context| {
            let mut cx = EventContext::new(context, cx.transform);
            self.view.event(&mut cx, event);
        });
    }

    pub fn layout_indexed(
        &self,
        index: usize,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        cx.context.child(index, |context| {
            let mut cx = LayoutContext::new(context);
            let size = self.view.layout(&mut cx, space);
            cx.tree.set_size(size);
            size
        })
    }

    pub fn draw_indexed(&self, index: usize, cx: &mut DrawContext<'_>) {
        cx.with_layer(1.0, |cx| {
            cx.context.child(index, |context| {
                let mut cx = DrawContext::new(context, cx.frame);
                self.view.draw(&mut cx);
            });
        });
    }

    pub fn event_padded(&self, cx: &mut EventContext<'_>, event: &Event, padding: Padding) {
        cx.with_padding(padding, |cx| self.event(cx, event));
    }

    pub fn layout_padded(
        &self,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
        padding: Padding,
    ) -> Vec2 {
        let size = padding.size(cx);
        self.layout(cx, space.shrink(size)) + size
    }

    pub fn draw_padded(&self, cx: &mut DrawContext<'_>, padding: Padding) {
        cx.with_padding(padding, |cx| self.draw(cx));
    }

    pub fn event(&self, cx: &mut EventContext<'_>, event: &Event) {
        self.event_indexed(0, cx, event);
    }

    pub fn layout(&self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        self.layout_indexed(0, cx, space)
    }

    pub fn draw(&self, cx: &mut DrawContext<'_>) {
        self.draw_indexed(0, cx);
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node").finish()
    }
}
