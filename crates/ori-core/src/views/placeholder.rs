use ori_graphics::{math::Vec2, Color, Quad};
use ori_reactive::Event;

use crate::{
    AvailableSpace, BorderRadius, BorderWidth, DrawContext, EventContext, Key, LayoutContext,
    Length, Size, Style, View,
};

#[derive(Clone, Debug)]
pub struct Placeholder {
    pub size: Size,
    pub color: Style<Color>,
    pub border_width: Style<BorderWidth>,
    pub border_radius: Style<BorderRadius>,
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

    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }

    pub fn color(mut self, color: impl Into<Style<Color>>) -> Self {
        self.color = color.into();
        self
    }

    pub fn border_width(mut self, border_width: impl Into<Style<BorderWidth>>) -> Self {
        self.border_width = border_width.into();
        self
    }

    pub fn border_radius(mut self, border_radius: impl Into<Style<BorderRadius>>) -> Self {
        self.border_radius = border_radius.into();
        self
    }

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
