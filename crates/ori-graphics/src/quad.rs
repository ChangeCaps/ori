use crate::{Color, ImageHandle, Rect};

/// A rectangle with rounded corners, and an optional border.
#[derive(Clone, Debug, PartialEq)]
pub struct Quad {
    /// The rectangle of the quad.
    pub rect: Rect,
    /// The background color of the quad.
    pub background_color: Color,
    /// The background image of the quad.
    pub background_image: Option<ImageHandle>,
    /// The radius of the quad's corners.
    ///
    /// The radius of each corner is specified in the following order:
    /// top-left, top-right, bottom-right, bottom-left.
    pub border_radius: [f32; 4],
    /// The width of the quad's borders.
    ///
    /// The width of each border is specified in the following order:
    /// top, right, bottom, left.
    pub border_width: [f32; 4],
    /// The color of the quad's border.
    pub border_color: Color,
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            rect: Rect::default(),
            background_color: Color::WHITE,
            background_image: None,
            border_radius: [0.0; 4],
            border_width: [0.0; 4],
            border_color: Color::BLACK,
        }
    }
}

impl Quad {
    /// Rounds the rectangle of the quad to the nearest integer.
    pub fn round(self) -> Self {
        Self {
            rect: self.rect.round(),
            ..self
        }
    }
}
