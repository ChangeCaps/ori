use ori_graphics::Color;

use crate::{Key, Theme};

pub const TEXT: Key<Color> = Key::new("text");
pub const BACKGROUND: Key<Color> = Key::new("background");
pub const PRIMARY: Key<Color> = Key::new("primary");
pub const SECONDARY: Key<Color> = Key::new("secondary");
pub const ACCENT: Key<Color> = Key::new("accent");

pub fn light_palette() -> Theme {
    let mut theme = Theme::new();

    theme.set(TEXT, Color::hsl(0.0, 0.0, 0.0));
    theme.set(BACKGROUND, Color::hsl(0.0, 0.0, 1.0));
    theme.set(PRIMARY, Color::hsl(221.0, 1.0, 0.78));
    theme.set(SECONDARY, Color::hsl(222.0, 1.0, 0.96));
    theme.set(ACCENT, Color::hsl(334.0, 0.76, 0.47));

    theme
}
