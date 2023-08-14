use ori_graphics::math::Vec2;
use ori_reactive::Event;

use crate::{AvailableSpace, DrawContext, EventContext, LayoutContext, Node, Padding, View};

/// A view that adds padding to its content.
#[derive(Debug)]
pub struct Pad {
    /// The content of the view.
    pub content: Node,
    /// The padding of the view.
    pub padding: Padding,
}

impl Pad {
    /// Create a new pad view.
    pub fn new(padding: impl Into<Padding>, view: impl Into<Node>) -> Self {
        Self {
            content: view.into(),
            padding: padding.into(),
        }
    }
}

impl View for Pad {
    fn event(&mut self, cx: &mut EventContext<'_>, event: &Event) {
        self.content.event(cx, event);
    }

    fn layout(&mut self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        self.content.layout_padded(cx, space, self.padding)
    }

    fn draw(&mut self, cx: &mut DrawContext<'_>) {
        self.content.draw(cx);
    }
}
