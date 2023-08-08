use glam::Vec2;
use ori_graphics::{
    Color, FontFamily, FontStretch, FontStyle, FontWeight, Glyphs, TextAlign, TextSection, TextWrap,
};
use ori_reactive::Event;

use crate::{AvailableSpace, DrawContext, EventContext, IntoView, LayoutContext, StateView};

macro_rules! impl_into_text {
    ($($ty:ty),*) => {$(
        impl From<$ty> for Text {
            fn from(value: $ty) -> Self {
                Self {
                    text: value.to_string(),
                }
            }
        }

        impl IntoView for $ty {
            type View = Text;

            fn into_view(self) -> Self::View {
                Text::new(self)
            }
        }
    )*};
}

impl_into_text!(
    bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, &str,
    String
);

#[derive(Clone, Debug, Default)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: impl Into<Text>) -> Self {
        text.into()
    }
}

impl StateView for Text {
    type State = Option<Glyphs>;

    fn build(&self) -> Self::State {
        None
    }

    fn event(&self, _state: &mut Self::State, _cx: &mut EventContext<'_>, _event: &Event) {}

    fn layout(
        &self,
        state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        let section = TextSection {
            text: &self.text,
            font_size: 16.0,
            font_family: FontFamily::Serif,
            font_weight: FontWeight::NORMAL,
            font_stretch: FontStretch::Normal,
            font_style: FontStyle::Normal,
            color: Color::BLACK,
            v_align: TextAlign::Start,
            h_align: TextAlign::Center,
            line_height: 1.0,
            wrap: TextWrap::Word,
            bounds: space.max,
        };

        *state = cx.fonts.layout_glyphs(&section);
        state.as_ref().map_or(space.min, |glyphs| glyphs.size())
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        if let Some(glyphs) = state {
            cx.draw_text(glyphs, cx.rect());
        }
    }
}
