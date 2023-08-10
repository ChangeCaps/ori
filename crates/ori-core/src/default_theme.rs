use ori_graphics::{Color, FontFamily, FontStretch, FontStyle, FontWeight, TextAlign, TextWrap};

use crate::{
    light_palette,
    views::{Button, Text},
    Theme, Unit, PRIMARY, TEXT,
};

fn default_text_theme(theme: &mut Theme) {
    theme.set(Text::FONT_SIZE, Unit::Em(1.0));
    theme.set(Text::FONT_FAMILY, FontFamily::SansSerif);
    theme.set(Text::FONT_WEIGHT, FontWeight::NORMAL);
    theme.set(Text::FONT_STRETCH, FontStretch::Normal);
    theme.set(Text::FONT_STYLE, FontStyle::Normal);
    theme.set(Text::COLOR, TEXT);
    theme.set(Text::V_ALIGN, TextAlign::Top);
    theme.set(Text::H_ALIGN, TextAlign::Left);
    theme.set(Text::LINE_HEIGHT, 1.0);
    theme.set(Text::WRAP, TextWrap::Word);
}

fn default_button_theme(theme: &mut Theme) {
    theme.set(Button::COLOR, PRIMARY);
    theme.set(Button::BORDER_WIDTH, [0.0; 4]);
    theme.set(Button::BORDER_RADIUS, [5.0; 4]);
    theme.set(Button::BORDER_COLOR, Color::TRANSPARENT);
}

pub fn default_theme() -> Theme {
    let mut theme = Theme::new();

    theme.extend(light_palette());

    default_text_theme(&mut theme);
    default_button_theme(&mut theme);

    theme
}
