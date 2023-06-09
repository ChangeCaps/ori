use glam::Vec2;
use ori_graphics::{Rect, TextSection};
use ori_macro::Build;
use ori_style::Style;

use crate::{AvailableSpace, Context, DrawContext, Element, LayoutContext, View};

macro_rules! impl_from {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for View {
                fn from(value: $ty) -> Self {
                    Self::new(Text::new(value))
                }
            }
        )*
    };
}

impl_from!(
    String, &str, char, bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32,
    f64
);

/// A text element.
#[derive(Clone, Debug, Default, Build)]
pub struct Text {
    /// The text to display.
    #[prop]
    pub text: String,
}

impl Text {
    /// Create a new text element.
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
        }
    }

    /// Set the text to display.
    pub fn text(mut self, text: impl ToString) -> Self {
        self.text = text.to_string();
        self
    }
}

impl Element for Text {
    type State = Option<f32>;

    fn build(&self) -> Self::State {
        None
    }

    fn style(&self) -> Style {
        Style::new("text")
    }

    fn layout(
        &self,
        state: &mut Self::State,
        cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        let font_size = cx.style_range("font-size", 0.0..cx.parent_space.max.y);
        *state = Some(font_size);

        let section = TextSection {
            text: &self.text,
            font_size,
            font_family: cx.style("font-family"),
            font_weight: cx.style("font-weight"),
            font_stretch: cx.style("font-stretch"),
            font_style: cx.style("font-style"),
            color: cx.style("color"),
            h_align: cx.style("text-align"),
            v_align: cx.style("text-valign"),
            line_height: cx.style("line-height"),
            wrap: cx.style("text-wrap"),
            rect: Rect::min_size(Vec2::ZERO, space.max),
        };

        let text_rect = cx.measure_text(&section);
        text_rect.size()
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext) {
        let font_size = state.unwrap_or(16.0);
        let section = TextSection {
            text: &self.text,
            font_size,
            font_family: cx.style("font-family"),
            font_weight: cx.style("font-weight"),
            font_stretch: cx.style("font-stretch"),
            font_style: cx.style("font-style"),
            color: cx.style("color"),
            h_align: cx.style("text-align"),
            v_align: cx.style("text-valign"),
            line_height: cx.style("line-height"),
            wrap: cx.style("text-wrap"),
            rect: cx.rect(),
        };

        cx.draw_text(&section);
    }
}
