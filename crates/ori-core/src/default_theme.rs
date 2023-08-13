use ori_graphics::{Color, FontFamily, FontStretch, FontStyle, FontWeight, TextAlign, TextWrap};

use crate::{
    views::{Button, CheckBox, Placeholder, Text},
    BorderRadius, BorderWidth, Key, Palette, Theme, Unit,
};

fn default_text_theme(theme: &mut Theme) {
    theme.set(Text::FONT_SIZE, Unit::Em(1.0));
    theme.set(Text::FONT_FAMILY, FontFamily::SansSerif);
    theme.set(Text::FONT_WEIGHT, FontWeight::NORMAL);
    theme.set(Text::FONT_STRETCH, FontStretch::Normal);
    theme.set(Text::FONT_STYLE, FontStyle::Normal);
    theme.set(Text::COLOR, Palette::TEXT);
    theme.set(Text::V_ALIGN, TextAlign::Top);
    theme.set(Text::H_ALIGN, TextAlign::Left);
    theme.set(Text::LINE_HEIGHT, 1.0);
    theme.set(Text::WRAP, TextWrap::Word);
}

fn default_icon_theme(theme: &mut Theme) {
    theme.set::<Unit>(Key::new("icon.size"), Unit::Em(1.0));
    theme.set::<Color>(Key::new("icon.color"), Color::BLACK);
}

fn default_button_theme(theme: &mut Theme) {
    theme.set(Button::COLOR, Palette::PRIMARY);
    theme.set(Button::BORDER_WIDTH, BorderWidth::ZERO);
    theme.set(Button::BORDER_RADIUS, BorderRadius::all(Unit::Em(0.5)));
    theme.set(Button::BORDER_COLOR, Color::TRANSPARENT);
}

fn default_check_box_theme(theme: &mut Theme) {
    theme.set(CheckBox::SIZE, Unit::Em(1.5));
    theme.set(CheckBox::COLOR, Palette::TEXT_BRIGHTER);
    theme.set(CheckBox::STROKE, Unit::Px(1.0));
    theme.set(CheckBox::BACKGROUND, Palette::BACKGROUND_DARK);
    theme.set(CheckBox::BORDER_WIDTH, BorderWidth::all(Unit::Px(1.5)));
    theme.set(CheckBox::BORDER_RADIUS, BorderRadius::all(Unit::Em(0.4)));
    theme.set(CheckBox::BORDER_COLOR, Palette::TEXT_BRIGHT);
}

fn default_placeholder_theme(theme: &mut Theme) {
    theme.set(Placeholder::COLOR, Palette::SECONDARY);
    theme.set(Placeholder::BORDER_WIDTH, BorderWidth::ZERO);
    theme.set(Placeholder::BORDER_RADIUS, BorderRadius::all(Unit::Em(0.5)));
    theme.set(Placeholder::BORDER_COLOR, Color::TRANSPARENT);
}

pub fn default_theme() -> Theme {
    let mut theme = Theme::new();

    default_text_theme(&mut theme);
    default_icon_theme(&mut theme);
    default_button_theme(&mut theme);
    default_check_box_theme(&mut theme);
    default_placeholder_theme(&mut theme);

    theme
}
