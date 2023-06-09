use glam::Vec2;
use ori_macro::Build;
use ori_reactive::{Emitter, Event};
use ori_style::Style;

use crate::{
    AvailableSpace, Children, Context, DrawContext, Element, EventContext, FlexLayout,
    LayoutContext, Parent, PointerEvent, View,
};

/// A button element.
#[derive(Default, Build)]
pub struct Button {
    /// On click callback.
    #[event]
    pub on_click: Emitter<PointerEvent>,
    /// The content of the button.
    #[children]
    pub children: Children,
}

impl Button {
    /// Create a new button.
    pub fn new(child: impl Into<View>) -> Self {
        Self {
            on_click: Emitter::new(),
            children: Children::new().with_child(child),
        }
    }

    fn handle_pointer_event(
        &self,
        cx: &mut EventContext,
        event: &PointerEvent,
        handled: bool,
    ) -> bool {
        if event.is_press() && cx.hovered() && !handled {
            if !self.on_click.is_empty() {
                cx.activate();
                self.on_click.emit(event);
            }
        } else if event.is_release() && cx.state.active {
            cx.deactivate();
        } else {
            return false;
        }

        true
    }
}

impl Element for Button {
    type State = ();

    fn build(&self) -> Self::State {}

    fn style(&self) -> Style {
        Style::new("button")
    }

    fn event(&self, _: &mut Self::State, cx: &mut EventContext, event: &Event) {
        self.children.event(cx, event);

        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if self.handle_pointer_event(cx, pointer_event, event.is_handled()) {
                event.handle();
            }
        }
    }

    fn layout(&self, _: &mut Self::State, cx: &mut LayoutContext, space: AvailableSpace) -> Vec2 {
        let flex = FlexLayout::from_style(cx);
        self.children.flex_layout(cx, space, flex)
    }

    fn draw(&self, _: &mut Self::State, cx: &mut DrawContext) {
        cx.draw_quad();
        self.children.draw(cx);
    }
}
