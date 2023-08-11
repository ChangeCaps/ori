use glam::Vec2;
use ori_graphics::{
    Color, FontFamily, FontStretch, FontStyle, FontWeight, Glyphs, TextAlign, TextSection, TextWrap,
};
use ori_reactive::Event;

use crate::{
    AvailableSpace, DrawContext, EventContext, Key, LayoutContext, Node, StateView, Style, Styled,
    Unit,
};

macro_rules! impl_into_text {
    ($($ty:ty),*) => {$(
        impl From<$ty> for Node {
            fn from(value: $ty) -> Self {
                Node::from(Text::from(value))
            }
        }
    )*};
}

impl_into_text!(
    bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, &str,
    String
);

impl<T: ToString> From<T> for Text {
    fn from(value: T) -> Self {
        Self {
            text: value.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Text {
    pub text: String,
    pub font_size: Style<Unit>,
    pub font_family: Style<FontFamily>,
    pub font_weight: Style<FontWeight>,
    pub font_stretch: Style<FontStretch>,
    pub font_style: Style<FontStyle>,
    pub color: Style<Color>,
    pub v_align: Style<TextAlign>,
    pub h_align: Style<TextAlign>,
    pub line_height: Style<f32>,
    pub wrap: Style<TextWrap>,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: Default::default(),
            font_size: Style::new(Self::FONT_SIZE),
            font_family: Style::new(Self::FONT_FAMILY),
            font_weight: Style::new(Self::FONT_WEIGHT),
            font_stretch: Style::new(Self::FONT_STRETCH),
            font_style: Style::new(Self::FONT_STYLE),
            color: Style::new(Self::COLOR),
            v_align: Style::new(Self::V_ALIGN),
            h_align: Style::new(Self::H_ALIGN),
            line_height: Style::new(Self::LINE_HEIGHT),
            wrap: Style::new(Self::WRAP),
        }
    }
}

impl Text {
    pub const FONT_SIZE: Key<Unit> = Key::new("text.font-size");
    pub const FONT_FAMILY: Key<FontFamily> = Key::new("text.font-family");
    pub const FONT_WEIGHT: Key<FontWeight> = Key::new("text.font-weight");
    pub const FONT_STRETCH: Key<FontStretch> = Key::new("text.font-stretch");
    pub const FONT_STYLE: Key<FontStyle> = Key::new("text.font-style");
    pub const COLOR: Key<Color> = Key::new("text.color");
    pub const V_ALIGN: Key<TextAlign> = Key::new("text.v-align");
    pub const H_ALIGN: Key<TextAlign> = Key::new("text.h-align");
    pub const LINE_HEIGHT: Key<f32> = Key::new("text.line-height");
    pub const WRAP: Key<TextWrap> = Key::new("text.wrap");

    pub fn new(text: impl Into<Text>) -> Self {
        text.into()
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn font_size(mut self, font_size: impl Styled<Unit>) -> Self {
        self.font_size = font_size.style();
        self
    }

    pub fn font_family(mut self, font_family: impl Styled<FontFamily>) -> Self {
        self.font_family = font_family.style();
        self
    }

    pub fn font_weight(mut self, font_weight: impl Styled<FontWeight>) -> Self {
        self.font_weight = font_weight.style();
        self
    }

    pub fn font_stretch(mut self, font_stretch: impl Styled<FontStretch>) -> Self {
        self.font_stretch = font_stretch.style();
        self
    }

    pub fn font_style(mut self, font_style: impl Styled<FontStyle>) -> Self {
        self.font_style = font_style.style();
        self
    }

    pub fn color(mut self, color: impl Styled<Color>) -> Self {
        self.color = color.style();
        self
    }

    pub fn v_align(mut self, v_align: impl Styled<TextAlign>) -> Self {
        self.v_align = v_align.style();
        self
    }

    pub fn h_align(mut self, h_align: impl Styled<TextAlign>) -> Self {
        self.h_align = h_align.style();
        self
    }

    pub fn line_height(mut self, line_height: impl Styled<f32>) -> Self {
        self.line_height = line_height.style();
        self
    }

    pub fn wrap(mut self, wrap: impl Styled<TextWrap>) -> Self {
        self.wrap = wrap.style();
        self
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
            font_size: cx.unit(self.font_size.get(cx.theme)),
            font_family: self.font_family.get(cx.theme),
            font_weight: self.font_weight.get(cx.theme),
            font_stretch: self.font_stretch.get(cx.theme),
            font_style: self.font_style.get(cx.theme),
            color: self.color.get(cx.theme),
            v_align: self.v_align.get(cx.theme),
            h_align: self.h_align.get(cx.theme),
            line_height: self.line_height.get(cx.theme),
            wrap: self.wrap.get(cx.theme),
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
