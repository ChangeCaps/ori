use glam::Vec2;
use ori_graphics::Rect;
use ori_reactive::Event;

use crate::{
    AvailableSpace, DrawContext, EventContext, LayoutContext, Length, Node, Size, StateView,
};

#[derive(Clone, Debug)]
pub struct Align {
    pub content: Node,
    pub size: Size,
    pub alignment: Vec2,
}

impl Align {
    pub fn new(alignment: impl Into<Vec2>, view: impl Into<Node>) -> Self {
        Self {
            content: Node::new(view),
            size: Size::parent(),
            alignment: alignment.into(),
        }
    }

    pub fn center(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(0.5, 0.5), view)
    }

    pub fn top_left(view: impl Into<Node>) -> Self {
        Self::new(Vec2::ZERO, view)
    }

    pub fn top(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(0.5, 0.0), view)
    }

    pub fn top_right(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(1.0, 0.0), view)
    }

    pub fn left(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(0.0, 0.5), view)
    }

    pub fn right(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(1.0, 0.5), view)
    }

    pub fn bottom_left(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(0.0, 1.0), view)
    }

    pub fn bottom(view: impl Into<Node>) -> Self {
        Self::new(Vec2::new(0.5, 1.0), view)
    }

    pub fn bottom_right(view: impl Into<Node>) -> Self {
        Self::new(Vec2::ONE, view)
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }

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

    fn build(&self) -> Self::State {
        Vec2::ZERO
    }

    fn event(&self, state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        cx.with_translation(self.contet_offset(state, cx.rect()), |cx| {
            self.content.event(cx, event);
        });
    }

    fn layout(
        &self,
        state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        let content = self.content.layout(cx, self.size.content_space(cx, space));
        *state = content;

        space.constrain(self.size.get(cx, content, space))
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        cx.with_translation(self.contet_offset(state, cx.rect()), |cx| {
            self.content.draw(cx);
        });
    }
}
