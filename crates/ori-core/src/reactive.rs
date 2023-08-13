use std::sync::mpsc::Receiver;

use ori_graphics::math::Vec2;
use ori_reactive::Event;

use crate::{AvailableSpace, DrawContext, EventContext, LayoutContext, Node, View};

/// A node that can be dynamically changed reactively.
///
/// See [`reactive`](crate::function::reactive) for more information.
#[derive(Debug)]
pub struct ReactiveNode {
    receiver: Receiver<Node>,
    content: Node,
}

impl ReactiveNode {
    /// Creates a new reactive node.
    ///
    /// See [`reactive`](crate::function::reactive) for more information.
    pub fn new(receiver: Receiver<Node>) -> Self {
        Self {
            receiver,
            content: Node::empty(),
        }
    }

    pub fn recv(&mut self) {
        if let Ok(content) = self.receiver.try_recv() {
            self.content = content;
        }
    }
}

impl View for ReactiveNode {
    fn event(&mut self, cx: &mut EventContext<'_>, event: &Event) {
        self.recv();
        self.content.event(cx, event);
    }

    fn layout(&mut self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        self.recv();
        self.content.layout(cx, space)
    }

    fn draw(&mut self, cx: &mut DrawContext<'_>) {
        self.recv();
        self.content.draw(cx);
    }
}
