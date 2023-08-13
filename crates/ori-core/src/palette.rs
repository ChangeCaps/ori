use ori_graphics::Color;

use crate::{Key, Theme};

/// A color palette.
#[derive(Clone, Copy, Debug)]
pub struct Palette {
    /// The text color.
    pub text: Color,
    /// The background color.
    pub background: Color,
    /// The primary color.
    pub primary: Color,
    /// The secondary color.
    pub secondary: Color,
    /// The accent color.
    pub accent: Color,
}

impl Palette {
    pub const TEXT: Key<Color> = Key::new("--text");
    pub const BACKGROUND: Key<Color> = Key::new("--background");
    pub const PRIMARY: Key<Color> = Key::new("--primary");
    pub const SECONDARY: Key<Color> = Key::new("--secondary");
    pub const ACCENT: Key<Color> = Key::new("--accent");

    pub const TEXT_DARK: Key<Color> = Key::new("--text-dark");
    pub const TEXT_DARKER: Key<Color> = Key::new("--text-darker");
    pub const TEXT_BRIGHT: Key<Color> = Key::new("--text-bright");
    pub const TEXT_BRIGHTER: Key<Color> = Key::new("--text-brighter");

    pub const BACKGROUND_DARK: Key<Color> = Key::new("--background-dark");
    pub const BACKGROUND_DARKER: Key<Color> = Key::new("--background-darker");
    pub const BACKGROUND_BRIGHT: Key<Color> = Key::new("--background-bright");
    pub const BACKGROUND_BRIGHTER: Key<Color> = Key::new("--background-brighter");

    pub const PRIMARY_DARK: Key<Color> = Key::new("--primary-dark");
    pub const PRIMARY_DARKER: Key<Color> = Key::new("--primary-darker");
    pub const PRIMARY_BRIGHT: Key<Color> = Key::new("--primary-bright");
    pub const PRIMARY_BRIGHTER: Key<Color> = Key::new("--primary-brighter");

    pub const SECONDARY_DARK: Key<Color> = Key::new("--secondary-dark");
    pub const SECONDARY_DARKER: Key<Color> = Key::new("--secondary-darker");
    pub const SECONDARY_BRIGHT: Key<Color> = Key::new("--secondary-bright");
    pub const SECONDARY_BRIGHTER: Key<Color> = Key::new("--secondary-brighter");

    pub const ACCENT_DARK: Key<Color> = Key::new("--accent-dark");
    pub const ACCENT_DARKER: Key<Color> = Key::new("--accent-darker");
    pub const ACCENT_BRIGHT: Key<Color> = Key::new("--accent-bright");
    pub const ACCENT_BRIGHTER: Key<Color> = Key::new("--accent-brighter");

    pub fn light() -> Self {
        Self {
            text: Color::hsl(0.0, 0.0, 0.2),
            background: Color::hsl(0.0, 0.0, 1.0),
            primary: Color::hsl(221.0, 1.0, 0.78),
            secondary: Color::hsl(0.0, 0.0, 0.98),
            accent: Color::hsl(334.0, 0.76, 0.47),
        }
    }

    pub fn dark() -> Self {
        Self {
            text: Color::hsl(0.0, 0.0, 0.8),
            background: Color::hsl(0.0, 0.0, 0.2),
            primary: Color::hsl(221.0, 1.0, 0.78),
            secondary: Color::hsl(0.0, 0.0, 0.98),
            accent: Color::hsl(334.0, 0.76, 0.47),
        }
    }

    pub fn to_theme(&self) -> Theme {
        let mut theme = Theme::new();

        theme.set(Self::TEXT, self.text);
        theme.set(Self::BACKGROUND, self.background);
        theme.set(Self::PRIMARY, self.primary);
        theme.set(Self::SECONDARY, self.secondary);
        theme.set(Self::ACCENT, self.accent);

        Self::derived_theme(&mut theme, 0.2);

        theme
    }

    fn derived_theme(theme: &mut Theme, f: f32) {
        darker(theme, Self::TEXT_DARKER, Self::TEXT, f);
        dark(theme, Self::TEXT_DARK, Self::TEXT, f);
        bright(theme, Self::TEXT_BRIGHT, Self::TEXT, f);
        brighter(theme, Self::TEXT_BRIGHTER, Self::TEXT, f);

        darker(theme, Self::BACKGROUND_DARKER, Self::BACKGROUND, f);
        dark(theme, Self::BACKGROUND_DARK, Self::BACKGROUND, f);
        bright(theme, Self::BACKGROUND_BRIGHT, Self::BACKGROUND, f);
        brighter(theme, Self::BACKGROUND_BRIGHTER, Self::BACKGROUND, f);

        darker(theme, Self::PRIMARY_DARKER, Self::PRIMARY, f);
        dark(theme, Self::PRIMARY_DARK, Self::PRIMARY, f);
        bright(theme, Self::PRIMARY_BRIGHT, Self::PRIMARY, f);
        brighter(theme, Self::PRIMARY_BRIGHTER, Self::PRIMARY, f);

        darker(theme, Self::SECONDARY_DARKER, Self::SECONDARY, f);
        dark(theme, Self::SECONDARY_DARK, Self::SECONDARY, f);
        bright(theme, Self::SECONDARY_BRIGHT, Self::SECONDARY, f);
        brighter(theme, Self::SECONDARY_BRIGHTER, Self::SECONDARY, f);

        darker(theme, Self::ACCENT_DARKER, Self::ACCENT, f);
        dark(theme, Self::ACCENT_DARK, Self::ACCENT, f);
        bright(theme, Self::ACCENT_BRIGHT, Self::ACCENT, f);
        brighter(theme, Self::ACCENT_BRIGHTER, Self::ACCENT, f);
    }
}

fn darker(theme: &mut Theme, t: Key<Color>, s: Key<Color>, factor: f32) {
    theme.map(t, s, move |color| color.darken(factor));
}

fn dark(theme: &mut Theme, t: Key<Color>, s: Key<Color>, factor: f32) {
    theme.map(t, s, move |color| color.darken(factor / 2.0));
}

fn bright(theme: &mut Theme, t: Key<Color>, s: Key<Color>, factor: f32) {
    theme.map(t, s, move |color| color.brighten(factor / 2.0));
}

fn brighter(theme: &mut Theme, t: Key<Color>, s: Key<Color>, factor: f32) {
    theme.map(t, s, move |color| color.brighten(factor));
}
