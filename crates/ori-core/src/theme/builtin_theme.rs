use ori_graphics::{Color, FontFamily, FontStretch, FontStyle, FontWeight, TextAlign, TextWrap};

use crate::{
    views::{Button, CheckBox, Placeholder, Radio, Text, TextInput},
    BorderRadius, BorderWidth, Key, Palette, Theme, Unit,
};

fn builtin_text_theme(theme: &mut Theme) {
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

fn builtin_text_input_theme(theme: &mut Theme) {
    theme.set(TextInput::FONT_SIZE, Unit::Em(1.0));
    theme.set(TextInput::FONT_FAMILY, FontFamily::SansSerif);
    theme.set(TextInput::FONT_WEIGHT, FontWeight::NORMAL);
    theme.set(TextInput::FONT_STRETCH, FontStretch::Normal);
    theme.set(TextInput::FONT_STYLE, FontStyle::Normal);
    theme.set(TextInput::COLOR, Palette::TEXT);
    theme.set(TextInput::V_ALIGN, TextAlign::Top);
    theme.set(TextInput::H_ALIGN, TextAlign::Left);
    theme.set(TextInput::LINE_HEIGHT, 1.0);
    theme.set(TextInput::WRAP, TextWrap::Word);
}

fn builtin_icon_theme(theme: &mut Theme) {
    theme.set::<Unit>(Key::new("icon.size"), Unit::Em(1.0));
    theme.set::<Color>(Key::new("icon.color"), Color::BLACK);
}

fn builtin_button_theme(theme: &mut Theme) {
    theme.set(Button::FLOAT, Unit::Em(0.2));
    theme.set(Button::COLOR, Palette::PRIMARY);
    theme.set(Button::BORDER_WIDTH, BorderWidth::ZERO);
    theme.set(Button::BORDER_RADIUS, BorderRadius::all(Unit::Em(0.5)));
    theme.set(Button::BORDER_COLOR, Color::TRANSPARENT);
}

fn builtin_check_box_theme(theme: &mut Theme) {
    theme.set(CheckBox::SIZE, Unit::Em(1.5));
    theme.set(CheckBox::COLOR, Palette::TEXT_BRIGHTER);
    theme.set(CheckBox::STROKE, Unit::Px(1.0));
    theme.set(CheckBox::BACKGROUND, Palette::BACKGROUND);
    theme.set(CheckBox::BORDER_WIDTH, BorderWidth::all(Unit::Px(1.5)));
    theme.set(CheckBox::BORDER_RADIUS, BorderRadius::all(Unit::Em(0.4)));
    theme.set(CheckBox::BORDER_COLOR, Palette::TEXT_BRIGHTER);
}

fn builtin_placeholder_theme(theme: &mut Theme) {
    theme.set(Placeholder::COLOR, Palette::SECONDARY);
    theme.set(Placeholder::BORDER_WIDTH, BorderWidth::ZERO);
    theme.set(Placeholder::BORDER_RADIUS, BorderRadius::all(Unit::Em(0.5)));
    theme.set(Placeholder::BORDER_COLOR, Color::TRANSPARENT);
}

fn builtin_radio_theme(theme: &mut Theme) {
    theme.set(Radio::RADIUS, Unit::Em(0.5));
    theme.set(Radio::COLOR, Palette::ACCENT);
    theme.set(Radio::BACKGROUND, Palette::BACKGROUND);
    theme.set(Radio::BORDER_WIDTH, Unit::Px(1.5));
    theme.set(Radio::BORDER_COLOR, Palette::TEXT_BRIGHTER);
}

pub fn builtin_theme() -> Theme {
    let mut theme = Theme::new();

    builtin_text_theme(&mut theme);
    builtin_text_input_theme(&mut theme);
    builtin_icon_theme(&mut theme);
    builtin_button_theme(&mut theme);
    builtin_check_box_theme(&mut theme);
    builtin_placeholder_theme(&mut theme);
    builtin_radio_theme(&mut theme);

    theme
}
