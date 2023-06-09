use glam::Vec2;
use ori_graphics::{Quad, Rect};
use ori_macro::Build;
use ori_reactive::Event;
use ori_style::Style;

use crate::{
    AvailableSpace, Children, Context, DrawContext, Element, EventContext, FlexLayout,
    LayoutContext, PointerEvent,
};

/// A combo box element.
#[derive(Clone, Debug, Default, Build)]
pub struct ComboBox {
    /// The title of the combo box.
    #[prop]
    pub title: Children,
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
        self.title.event(cx, event);

        if cx.active() {
            self.children.event(cx, event);
        }

        if event.is_handled() {
            return;
        }

        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if pointer_event.is_press() && cx.hovered() && !cx.active() {
                cx.activate();
            } else if pointer_event.is_press() && cx.active() {
                cx.deactivate();
            } else {
                return;
            }

            event.handle();
        }
    }

    fn layout(
        &self,
        state: &mut Self::State,
        cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        let flex = FlexLayout::from_style(cx);
        let title_size = self.title.flex_layout(cx, space, flex);

        if cx.active() {
            let content_space = AvailableSpace {
                min: Vec2::new(space.min.x, 0.0),
                max: Vec2::new(space.max.x, cx.window.size.y as f32),
            };

            let flex = FlexLayout::default();
            *state = self.children.flex_layout(cx, content_space, flex);
            self.children.set_offset(Vec2::new(0.0, title_size.y));
        }

        title_size
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext) {
        // draw the quad for the title
        cx.draw_quad();

        // draw the title
        cx.draw_layer(|cx| {
            self.title.draw(cx);
        });

        if cx.active() {
            let content_rect = Rect::min_size(cx.rect().bottom_left(), *state);
            let range = 0.0..content_rect.size().min_element();

            let content_quad = Quad {
                rect: content_rect,
                background: cx.style_group(&["content-background-color", "content-background"]),
                border_radius: [cx.style_range("content-border-radius", range.clone()); 4],
                border_width: cx.style_range("content-border-width", range),
                border_color: cx.style("content-border-color"),
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
