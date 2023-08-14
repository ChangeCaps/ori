use ori_graphics::{math::Vec2, Affine, Frame};
use ori_reactive::Event;

use crate::{AvailableSpace, Context, DrawContext, EventContext, LayoutContext, Node};

impl Node {
    pub(crate) fn event_root(&mut self, context: Context<'_>, event: &Event) {
        let mut cx = EventContext::new(context, Affine::IDENTITY);
        self.event(&mut cx, event);
    }

    pub(crate) fn layout_root(&mut self, context: Context<'_>, space: AvailableSpace) -> Vec2 {
        let mut cx = LayoutContext::new(context);
        self.layout(&mut cx, space)
    }

    pub(crate) fn draw_root(&mut self, context: Context<'_>, frame: &mut Frame) {
        let mut cx = DrawContext::new(context, frame);
        self.draw(&mut cx);
    }
}
