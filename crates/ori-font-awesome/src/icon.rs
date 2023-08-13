use ori_core::{
    AvailableSpace, Context, DrawContext, EventContext, Key, LayoutContext, StateView, Style,
    Styled, Unit,
};
use ori_graphics::{
    math::Vec2, Color, FontFamily, FontQuery, FontStretch, FontStyle, FontWeight, Glyphs,
    TextAlign, TextSection, TextWrap,
};
use ori_reactive::Event;

use crate::{IconFont, IconKind};

const REGULAR: &[u8] = include_bytes!("../font/Font Awesome 6 Free-Regular-400.otf");
const SOLID: &[u8] = include_bytes!("../font/Font Awesome 6 Free-Solid-900.otf");
const BRAND: &[u8] = include_bytes!("../font/Font Awesome 6 Brands-Regular-400.otf");

/// A view that displays a single icon.
///
/// By default, the icon is rendered using the `icon.font` font family.
/// This uses the [Font Awesome 6 Regular Free](https://fontawesome.com/) font by default.
pub struct Icon {
    /// The codepoint of the icon to display.
    pub icon: IconKind,
    /// The size of the icon.
    pub size: Style<Unit>,
    /// The color of the icon.
    pub color: Style<Color>,
}

impl Icon {
    /// The size of the icon.
    pub const SIZE: Key<Unit> = Key::new("icon.size");
    /// The font family to use for the icon.
    pub const FONT: Key<FontFamily> = Key::new("icon.font");
    /// The regular font family to use for the icon.
    pub const SOLID: Key<FontFamily> = Key::new("icon.solid");
    /// The brand font family to use for the icon.
    pub const BRAND: Key<FontFamily> = Key::new("icon.brand");
    /// The color of the icon.
    pub const COLOR: Key<Color> = Key::new("icon.color");

    /// Create a new icon view.
    pub fn new(icon: impl Into<IconKind>) -> Self {
        Self {
            icon: icon.into(),
            size: Style::new(Self::SIZE),
            color: Style::new(Self::COLOR),
        }
    }

    /// Set the size of the icon.
    pub fn size(mut self, size: impl Styled<Unit>) -> Self {
        self.size = size.style();
        self
    }

    /// Set the color of the icon.
    pub fn color(mut self, color: impl Styled<Color>) -> Self {
        self.color = color.style();
        self
    }
}

impl StateView for Icon {
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
        let query = FontQuery {
            family: self.icon.font().family(),
            weight: FontWeight::NORMAL,
            stretch: FontStretch::Normal,
            style: FontStyle::Normal,
        };

        if cx.fonts.query(&query).is_none() {
            match self.icon.font() {
                IconFont::Regular => cx.fonts.load_font_data(REGULAR.to_vec()),
                IconFont::Solid => cx.fonts.load_font_data(SOLID.to_vec()),
                IconFont::Brand => cx.fonts.load_font_data(BRAND.to_vec()),
            }
        }

        let mut buffer = [0; 4];

        let section = TextSection {
            text: self.icon.code_point().encode_utf8(&mut buffer),
            font_size: self.size.get(cx.theme).get(cx),
            font_family: self.icon.font().family(),
            font_weight: FontWeight::NORMAL,
            font_stretch: FontStretch::Normal,
            font_style: FontStyle::Normal,
            color: self.color.get(cx.theme),
            v_align: TextAlign::Center,
            h_align: TextAlign::Center,
            line_height: 1.0,
            wrap: TextWrap::None,
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
