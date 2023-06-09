use glam::Vec2;
use ori_graphics::{Quad, Rect};
use ori_macro::Build;
use ori_reactive::{Emitter, Event, OwnedSignal};
use ori_style::Style;

use crate::{
    AvailableSpace, Children, Context, DrawContext, Element, EventContext, FlexLayout,
    LayoutContext, Padding, PointerEvent,
};

/// A combo box element.
#[derive(Clone, Debug, Default, Build)]
pub struct ComboBox {
    /// The title of the combo box.
    #[prop]
    pub title: Children,
    /// Whether the combo box is open.
    #[prop]
    #[bind]
    pub open: OwnedSignal<bool>,
    /// On click callback.
    #[event]
    pub on_click: Emitter<PointerEvent>,
    /// The children of the combo box.
    #[children]
    pub children: Children,
}

impl ComboBox {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Element for ComboBox {
    type State = Vec2;

    fn build(&self) -> Self::State {
        Vec2::ZERO
    }

    fn style(&self) -> Style {
        Style::new("combobox")
    }

    fn event(&self, _state: &mut Self::State, cx: &mut EventContext, event: &Event) {
        cx.state.active = self.open.get();

        self.title.event(cx, event);

        if cx.active() {
            self.children.event(cx, event);
        }

        if event.is_handled() {
            return;
        }

        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if pointer_event.is_press() && cx.hovered() {
                self.on_click.emit(pointer_event);
            }

            if pointer_event.is_press() && cx.hovered() && !cx.active() {
                self.open.set(true);
                cx.request_redraw();
                event.handle();
            } else if pointer_event.is_press() && cx.active() {
                self.open.set(false);
                cx.request_redraw();
                event.handle();
            }
        }
    }

    fn layout(
        &self,
        state: &mut Self::State,
        cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        let flex = FlexLayout {
            padding: Padding::from_style_named(cx, space, "title-padding"),
            axis: cx.style("title-direction"),
            justify_content: cx.style("title-justify-content"),
            align_items: cx.style("title-align-items"),
            gap: cx.style("title-gap"),
        };

        let title_size = self.title.flex_layout(cx, space, flex);

        if cx.active() {
            let content_space = AvailableSpace {
                min: Vec2::new(title_size.x, 0.0),
                max: cx.window.size.as_vec2(),
            };

            let flex = FlexLayout {
                padding: Padding::from_style(cx, content_space),
                ..FlexLayout::from_style(cx)
            };

            *state = self.children.flex_layout(cx, content_space, flex);
            let offset = Vec2::new(0.0, title_size.y) + flex.padding.offset();
            self.children.set_offset(offset);
        }

        title_size
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext) {
        // draw the quad for the title
        let title_quad = Quad {
            rect: cx.rect(),
            background: cx.style_group(&["title-background-color", "title-background"]),
            border_radius: cx.style_border_radius("title-border", cx.parent_size),
            border_width: cx.style_range("title-border-width", 0.0..cx.parent_size.min_element()),
            border_color: cx.style("title-border-color"),
        };

        cx.draw(title_quad);

        // draw the title
        cx.draw_layer(|cx| {
            self.title.draw(cx);
        });

        if cx.active() {
            let content_rect = Rect::min_size(cx.rect().bottom_left(), *state);
            let range = 0.0..content_rect.size().min_element();

            let content_quad = Quad {
                rect: content_rect,
                background: cx.style_group(&["background-color", "background"]),
                border_radius: cx.style_border_radius("border", content_rect.size()),
                border_width: cx.style_range("border-width", range),
                border_color: cx.style("border-color"),
            };

            cx.layer().z_index(1000.0).draw(|cx| {
                cx.draw(content_quad);

                cx.draw_layer(|cx| {
                    self.children.draw(cx);
                });
            });
        }
    }
}
