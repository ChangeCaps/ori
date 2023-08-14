use ori_graphics::{math::Vec2, Color, Quad};
use ori_reactive::Event;

use crate::{
    AvailableSpace, BorderRadius, BorderWidth, DrawContext, EventContext, Key, LayoutContext,
    Length, Size, Style, View,
};

/// A placeholder view.
///
/// This is useful for testing layouts.
#[derive(Clone, Debug)]
pub struct Placeholder {
    /// The size of the placeholder.
    pub size: Size,
    /// The color of the placeholder.
    pub color: Style<Color>,
    /// The border width of the placeholder.
    pub border_width: Style<BorderWidth>,
    /// The border radius of the placeholder.
    pub border_radius: Style<BorderRadius>,
    /// The border color of the placeholder.
    pub border_color: Style<Color>,
}

impl Default for Placeholder {
    fn default() -> Self {
        Self {
            size: Size::parent(),
            color: Style::new(Self::COLOR),
            border_width: Style::new(Self::BORDER_WIDTH),
            border_radius: Style::new(Self::BORDER_RADIUS),
            border_color: Style::new(Self::BORDER_COLOR),
        }
    }
}

impl Placeholder {
    pub const COLOR: Key<Color> = Key::new("placeholder.color");
    pub const BORDER_WIDTH: Key<BorderWidth> = Key::new("placeholder.border-width");
    pub const BORDER_RADIUS: Key<BorderRadius> = Key::new("placeholder.border-radius");
    pub const BORDER_COLOR: Key<Color> = Key::new("placeholder.border-color");

    /// Create a new placeholder view.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the size.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Set the width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    /// Set the height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }

    /// Set the color.
    pub fn color(mut self, color: impl Into<Style<Color>>) -> Self {
        self.color = color.into();
        self
    }

    /// Set the border width.
    pub fn border_width(mut self, border_width: impl Into<Style<BorderWidth>>) -> Self {
        self.border_width = border_width.into();
        self
    }

    /// Set the border radius.
    pub fn border_radius(mut self, border_radius: impl Into<Style<BorderRadius>>) -> Self {
        self.border_radius = border_radius.into();
        self
    }

    /// Set the border color.
    pub fn border_color(mut self, border_color: impl Into<Style<Color>>) -> Self {
        self.border_color = border_color.into();
        self
    }
}

impl View for Placeholder {
    fn event(&mut self, _cx: &mut EventContext<'_>, _event: &Event) {}

    fn layout(&mut self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        self.size.get(cx, Vec2::ZERO, space)
    }

    fn draw(&mut self, cx: &mut DrawContext<'_>) {
        cx.draw(Quad {
            rect: cx.rect(),
            background_color: self.color.get(cx.theme),
            background_image: None,
            border_radius: self.border_radius.get(cx.theme).get(cx),
            border_width: self.border_width.get(cx.theme).get(cx),
            border_color: self.border_color.get(cx.theme),
        });
    }
}
