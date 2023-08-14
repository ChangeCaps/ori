use ori_graphics::{
    math::Vec2, Color, FontFamily, FontStretch, FontStyle, FontWeight, Glyphs, TextAlign,
    TextSection, TextWrap,
};
use ori_reactive::Event;

use crate::{
    AvailableSpace, Context, DrawContext, EventContext, Key, LayoutContext, Node, StateView, Style,
    Styled, Unit,
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

/// A text view.
#[derive(Clone, Debug)]
pub struct Text {
    /// The text to display.
    pub text: String,
    /// The font size of the text.
    pub font_size: Style<Unit>,
    /// The font family of the text.
    pub font_family: Style<FontFamily>,
    /// The font weight of the text.
    pub font_weight: Style<FontWeight>,
    /// The font stretch of the text.
    pub font_stretch: Style<FontStretch>,
    /// The font style of the text.
    pub font_style: Style<FontStyle>,
    /// The color of the text.
    pub color: Style<Color>,
    /// The vertical alignment of the text.
    pub v_align: Style<TextAlign>,
    /// The horizontal alignment of the text.
    pub h_align: Style<TextAlign>,
    /// The line height of the text.
    pub line_height: Style<f32>,
    /// The text wrap of the text.
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

    /// Create a new text view.
    pub fn new(text: impl Into<Text>) -> Self {
        text.into()
    }

    /// Set the font size.
    pub fn font_size(mut self, font_size: impl Styled<Unit>) -> Self {
        self.font_size = font_size.style();
        self
    }

    /// Set the font family.
    pub fn font_family(mut self, font_family: impl Styled<FontFamily>) -> Self {
        self.font_family = font_family.style();
        self
    }

    /// Set the font weight.
    pub fn font_weight(mut self, font_weight: impl Styled<FontWeight>) -> Self {
        self.font_weight = font_weight.style();
        self
    }

    /// Set the font stretch.
    pub fn font_stretch(mut self, font_stretch: impl Styled<FontStretch>) -> Self {
        self.font_stretch = font_stretch.style();
        self
    }

    /// Set the font style.
    pub fn font_style(mut self, font_style: impl Styled<FontStyle>) -> Self {
        self.font_style = font_style.style();
        self
    }

    /// Set the color.
    pub fn color(mut self, color: impl Styled<Color>) -> Self {
        self.color = color.style();
        self
    }

    /// Set the vertical alignment.
    pub fn v_align(mut self, v_align: impl Styled<TextAlign>) -> Self {
        self.v_align = v_align.style();
        self
    }

    /// Set the horizontal alignment.
    pub fn h_align(mut self, h_align: impl Styled<TextAlign>) -> Self {
        self.h_align = h_align.style();
        self
    }

    /// Set the line height.
    pub fn line_height(mut self, line_height: impl Styled<f32>) -> Self {
        self.line_height = line_height.style();
        self
    }

    /// Set the text wrap.
    pub fn wrap(mut self, wrap: impl Styled<TextWrap>) -> Self {
        self.wrap = wrap.style();
        self
    }
}

impl StateView for Text {
    type State = Option<Glyphs>;

    fn build(&mut self, _cx: &mut Context<'_>) -> Self::State {
        None
    }

    fn event(&mut self, _state: &mut Self::State, _cx: &mut EventContext<'_>, _event: &Event) {}

    fn layout(
        &mut self,
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

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        if let Some(glyphs) = state {
            cx.draw_text(glyphs, cx.rect());
        }
    }
}
