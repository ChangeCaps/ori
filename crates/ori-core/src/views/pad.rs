use glam::Vec2;
use ori_reactive::Event;

use crate::{AvailableSpace, DrawContext, EventContext, LayoutContext, Node, Padding, View};

#[derive(Clone, Debug)]
pub struct Pad {
    pub content: Node,
    pub padding: Padding,
}

impl Pad {
    pub fn new(padding: impl Into<Padding>, view: impl Into<Node>) -> Self {
        Self {
            content: view.into(),
            padding: padding.into(),
        }
    }
}

impl View for Pad {
    fn event(&self, cx: &mut EventContext<'_>, event: &Event) {
        self.content.event_padded(cx, event, self.padding);
    }

    fn layout(&self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        self.content.layout_padded(cx, space, self.padding)
    }

    fn draw(&self, cx: &mut DrawContext<'_>) {
        self.content.draw_padded(cx, self.padding);
    }
}
