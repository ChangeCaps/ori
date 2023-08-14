use ori_graphics::{math::Vec2, Rect};
use ori_reactive::Event;

use crate::{
    AvailableSpace, Context, DrawContext, EventContext, LayoutContext, Length, Node, Size,
    StateView,
};

/// A view that aligns its content.
#[derive(Debug)]
pub struct Align {
    /// The content of the view.
    pub content: Node,
    /// The size of the view.
    pub size: Size,
    /// The alignment of the view.
    pub alignment: Vec2,
}

impl Align {
    /// Create a new align view.
    pub fn new(alignment: impl Into<Vec2>, view: impl Into<Node>) -> Self {
        Self {
            content: Node::new(view),
            size: Size::parent(),
            alignment: alignment.into(),
        }
    }

    /// Create a new align at the center.
    pub fn center(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(0.5, 0.5), view)
    }

    /// Create a new align at the top left.
    pub fn top_left(view: impl Into<Node>) -> Self {
        Self::new(Vec2::ZERO, view)
    }

    /// Create a new align at the top.
    pub fn top(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(0.5, 0.0), view)
    }

    /// Create a new align at the top right.
    pub fn top_right(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(1.0, 0.0), view)
    }

    /// Create a new align at the left.
    pub fn left(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(0.0, 0.5), view)
    }

    /// Create a new align at the right.
    pub fn right(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(1.0, 0.5), view)
    }

    /// Create a new align at the bottom left.
    pub fn bottom_left(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(0.0, 1.0), view)
    }

    /// Create a new align at the bottom.
    pub fn bottom(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(0.5, 1.0), view)
    }

    /// Create a new align at the bottom right.
    pub fn bottom_right(view: impl Into<Node>) -> Self {
        Self::new(Vec2::ONE, view)
    }

    /// Set the size.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Set the width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    /// Set the height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }

    /// Set the alignment.
    pub fn align(mut self, alignment: impl Into<Vec2>) -> Self {
        self.alignment = alignment.into();
        self
    }

    fn contet_offset(&self, state: &mut Vec2, rect: Rect) -> Vec2 {
        (rect.size() - *state) * self.alignment
    }
}

impl StateView for Align {
    type State = Vec2;

    fn build(&mut self, _cx: &mut Context<'_>) -> Self::State {
        Vec2::ZERO
    }

    fn event(&mut self, state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        cx.with_translation(self.contet_offset(state, cx.rect()), |cx| {
            self.content.event(cx, event);
        });
    }

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        let content = self.content.layout(cx, self.size.content_space(cx, space));
        *state = content;

        space.fit(self.size.get(cx, content, space))
    }

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        cx.with_translation(self.contet_offset(state, cx.rect()), |cx| {
            self.content.draw(cx);
        });
    }
}
