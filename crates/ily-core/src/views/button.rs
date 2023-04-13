use glam::Vec2;

use crate::{
    BoxConstraints, Div, DivEvents, DivProperties, DrawContext, Event, EventContext, Events,
    LayoutContext, Parent, PointerEvent, Properties, Scope, StyleClass, StyleClasses, View,
};

pub struct Button {
    pub content: Div,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            content: Div::new(),
        }
    }
}

impl Button {
    pub fn new(view: impl View) -> Self {
        Self::default().child(view)
    }

    pub fn child(mut self, view: impl View) -> Self {
        self.content = self.content.child(view);
        self
    }

    pub fn class(mut self, class: impl Into<StyleClass>) -> Self {
        self.content = self.content.class(class);
        self
    }

    pub fn on_press<'a>(mut self, cx: Scope<'a>, callback: impl FnMut(&PointerEvent) + 'a) -> Self {
        self.content = self.content.on_press(cx, callback);
        self
    }
}

impl Parent for Button {
    fn add_child(&mut self, view: impl View) {
        self.content.add_child(view);
    }
}

impl Properties for Button {
    type Setter<'a> = DivProperties<'a>;

    fn setter(&mut self) -> Self::Setter<'_> {
        Properties::setter(&mut self.content)
    }
}

impl Events for Button {
    type Setter<'a> = DivEvents<'a>;

    fn setter(&mut self) -> Self::Setter<'_> {
        Events::setter(&mut self.content)
    }
}

impl View for Button {
    type State = <Div as View>::State;

    fn build(&self) -> Self::State {
        self.content.build()
    }

    fn element(&self) -> Option<&'static str> {
        Some("button")
    }

    fn classes(&self) -> StyleClasses {
        self.content.classes()
    }

    fn event(&self, state: &mut Self::State, cx: &mut EventContext, event: &Event) {
        self.content.event(state, cx, event);
    }

    fn layout(&self, state: &mut Self::State, cx: &mut LayoutContext, bc: BoxConstraints) -> Vec2 {
        self.content.layout(state, cx, bc)
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext) {
        self.content.draw(state, cx);
    }
}