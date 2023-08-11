use glam::Vec2;
use ori_reactive::{Event, OwnedSignal};

use crate::{AvailableSpace, DrawContext, EventContext, LayoutContext, Node, View};

/// A node that can be dynamically changed reactively.
///
/// See [`reactive`](crate::function::reactive) for more information.
#[derive(Clone, Debug)]
pub struct ReactiveNode {
    signal: OwnedSignal<Node>,
}

impl ReactiveNode {
    /// Creates a new dynamic node.
    ///
    /// See [`reactive`](crate::function::reactive) for more information.
    pub fn new(signal: OwnedSignal<Node>) -> Self {
        Self { signal }
    }
}

impl View for ReactiveNode {
    fn event(&self, cx: &mut EventContext<'_>, event: &Event) {
        self.signal.get().event(cx, event);
    }

    fn layout(&self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        self.signal.get().layout(cx, space)
    }

    fn draw(&self, cx: &mut DrawContext<'_>) {
        self.signal.get().draw(cx);
    }
}
