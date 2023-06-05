use glam::Vec2;
use ori_reactive::Event;
use ori_style::Style;

use crate::{
    AvailableSpace, Div, DrawContext, Element, EventContext, Events, IntoView, LayoutContext,
    Parent, View,
};

/// A button element.
#[derive(Default)]
pub struct Button {
    /// The content of the button.
    pub content: Div,
}

impl Button {
    /// Create a new button.
    pub fn new(child: impl IntoView) -> Self {
        Self {
            content: Div::new().with_child(child),
        }
    }
}

impl Events for Button {
    type Setter<'a> = <Div as Events>::Setter<'a>;

    fn setter(&mut self) -> Self::Setter<'_> {
        Events::setter(&mut self.content)
    }
}

impl Parent for Button {
    type Child = <Div as Parent>::Child;

    fn clear_children(&mut self) {
        self.content.clear_children();
    }

    fn add_children(&mut self, child: impl Iterator<Item = View<Self::Child>>) -> usize {
        self.content.add_children(child)
    }

    fn set_children(&mut self, slot: usize, child: impl Iterator<Item = View<Self::Child>>) {
        self.content.set_children(slot, child)
    }
}

impl Element for Button {
    type State = <Div as Element>::State;

    fn build(&self) -> Self::State {}

    fn style(&self) -> Style {
        Style::new("button")
    }

    fn event(&self, state: &mut Self::State, cx: &mut EventContext, event: &Event) {
        self.content.event(state, cx, event);
    }

    fn layout(
        &self,
        state: &mut Self::State,
        cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        self.content.layout(state, cx, space)
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext) {
        self.content.draw(state, cx);
    }
}
