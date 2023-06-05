use glam::Vec2;
use ori_graphics::{Color, Quad};
use ori_macro::Build;
use ori_reactive::{Emitter, Event};
use ori_style::Style;

use crate::{
    AvailableSpace, Context, DrawContext, Element, EventContext, LayoutContext, PointerEvent,
};

/// A radio button element.
#[derive(Clone, Debug, Default, Build)]
pub struct Radio {
    /// Whether the radio button is selected.
    #[prop]
    pub selected: bool,
    /// The callback to be invoked when the radio button is clicked.
    #[event]
    pub on_click: Emitter<PointerEvent>,
}

impl Radio {
    /// Creates a new radio button element.
    pub fn new(selected: bool) -> Self {
        Self {
            selected,
            ..Default::default()
        }
    }
}

impl Element for Radio {
    type State = ();

    fn build(&self) -> Self::State {}

    fn style(&self) -> Style {
        Style::new("radio")
    }

    fn event(&self, _: &mut Self::State, cx: &mut EventContext, event: &Event) {
        cx.state.active = self.selected;

        if event.is_handled() || !cx.hovered() {
            return;
        }

        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if pointer_event.is_press() {
                self.on_click.emit(pointer_event);
                event.handle();
            }
        }
    }

    fn layout(&self, _: &mut Self::State, _: &mut LayoutContext, space: AvailableSpace) -> Vec2 {
        space.max
    }

    fn draw(&self, _: &mut Self::State, cx: &mut DrawContext) {
        cx.draw_quad();

        if self.selected {
            let rect = cx.rect().shrink(4.0);
            let quad = Quad {
                rect,
                background: cx.style("color"),
                border_radius: [rect.size().min_element() / 2.0; 4],
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            };

            cx.draw(quad);
        }
    }
}
