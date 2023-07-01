use glam::Vec2;
use ori_graphics::{FontFamily, TextAlign, TextSection};
use ori_macro::Build;
use ori_reactive::{Emitter, Event, OwnedSignal, Signal};
use ori_style::Style;

use crate::{
    AvailableSpace, Context, DrawContext, Element, EventContext, LayoutContext, PointerEvent,
};

/// A checkbox element.
#[derive(Default, Build)]
pub struct Checkbox {
    /// Whether the checkbox is checked.
    #[prop]
    #[bind]
    pub checked: OwnedSignal<bool>,
    /// On click callback.
    #[event]
    pub on_click: Emitter<PointerEvent>,
}

impl Checkbox {
    const CHECKMARK: &'static str = "\u{e876}";

    /// Create a new checkbox.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the checked state of the checkbox.
    pub fn checked(self, checked: bool) -> Self {
        self.checked.set(checked);
        self
    }

    /// Bind the checked state of the checkbox to a signal.
    pub fn bind_checked(mut self, binding: Signal<bool>) -> Self {
        self.checked.bind(binding);
        self
    }
}

impl Element for Checkbox {
    type State = ();

    fn build(&self) -> Self::State {}

    fn style(&self) -> Style {
        Style::new("checkbox")
    }

    fn event(&self, _: &mut Self::State, cx: &mut EventContext, event: &Event) {
        cx.state.active = self.checked.get();

        if event.is_handled() || !cx.hovered() {
            return;
        }

        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if pointer_event.is_press() {
                self.checked.set(!self.checked.get());
                self.on_click.emit(pointer_event);
                event.handle();
                cx.request_redraw();
            }
        }
    }

    fn layout(&self, _: &mut Self::State, _: &mut LayoutContext, space: AvailableSpace) -> Vec2 {
        space.max
    }

    fn draw(&self, _: &mut Self::State, cx: &mut DrawContext) {
        cx.state.active = self.checked.get();

        cx.draw_quad();

        if self.checked.get() {
            let section = TextSection {
                text: Self::CHECKMARK,
                font_size: cx.rect().size().min_element() * 0.8,
                font_family: FontFamily::Name(String::from("Material Icons")),
                h_align: TextAlign::Center,
                v_align: TextAlign::Center,
                color: cx.style("color"),
                bounds: cx.size(),
                ..Default::default()
            };

            if let Some(glyphs) = cx.layout_text(&section) {
                cx.draw_text(&glyphs, cx.rect());
            }
        }
    }
}
