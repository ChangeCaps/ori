use glam::Vec2;
use ori_macro::Build;
use ori_reactive::{Emitter, Event};
use ori_style::Style;

use crate::{
    AvailableSpace, Children, Context, DrawContext, Element, EventContext, FlexLayout,
    LayoutContext, PointerEvent,
};

/// A selectable element.
#[derive(Clone, Debug, Default, Build)]
pub struct Select {
    /// Whether the selectable element is selected.
    #[prop]
    pub selected: bool,
    #[event]
    pub on_click: Emitter<PointerEvent>,
    /// The children of the selectable element.
    #[children]
    pub children: Children,
}

impl Element for Select {
    type State = ();

    fn build(&self) -> Self::State {}

    fn style(&self) -> Style {
        Style::new("select")
    }

    fn event(&self, _state: &mut Self::State, cx: &mut EventContext, event: &Event) {
        self.children.event(cx, event);

        if event.is_handled() {
            return;
        }

        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if pointer_event.is_press() && cx.hovered() {
                self.on_click.emit(pointer_event);

                if !self.on_click.is_empty() {
                    event.handle();
                }
            }
        }
    }

    fn layout(
        &self,
        _state: &mut Self::State,
        cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        cx.state.active = self.selected;

        let flex = FlexLayout::from_style(cx);
        self.children.flex_layout(cx, space, flex)
    }

    fn draw(&self, _state: &mut Self::State, cx: &mut DrawContext) {
        cx.draw_quad();
        cx.draw_layer(|cx| {
            self.children.draw(cx);
        });
    }
}
