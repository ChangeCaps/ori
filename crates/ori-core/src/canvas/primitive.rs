use crate::layout::{Point, Rect};

use super::{Background, BorderRadius, BorderWidth, Color, Mesh};

/// A quad primitive.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Quad {
    /// The rectangle of the quad.
    pub rect: Rect,
    /// The color of the quad.
    pub background: Background,
    /// The border radius of the quad.
    pub border_radius: BorderRadius,
    /// The border width of the quad.
    pub border_width: BorderWidth,
    /// The border color of the quad.
    pub border_color: Color,
}

impl Quad {
    /// Get whether the quad is ineffective, i.e. it has no effect on the canvas.
    pub fn is_ineffective(&self) -> bool {
        // if the rect has zero area, the quad is ineffective
        let area_zero = self.rect.area() == 0.0;

        let background_transparent = self.background.color.is_transparent();

        let border_zero = self.border_width == BorderWidth::ZERO;
        let border_transparent = self.border_color.is_transparent();

        let border_ineffective = border_zero || border_transparent;

        area_zero || (background_transparent && border_ineffective)
    }
}

/// A primitive to be rendered.
#[derive(Clone, Debug)]
pub enum Primitive {
    /// A trigger primitive.
    Trigger(Rect),
    /// A quad primitive.
    Quad(Quad),
    /// A mesh primitive.
    Mesh(Mesh),
}

impl Primitive {
    /// Get whether the primitive is ineffective, i.e. it has no effect on the canvas.
    pub fn is_ineffective(&self) -> bool {
        match self {
            Self::Quad(quad) => quad.is_ineffective(),
            _ => false,
        }
    }

    /// Get whether the primitive intersects with the given point.
    pub fn intersects_point(&self, point: Point) -> bool {
        match self {
            Self::Trigger(rect) => rect.contains(point),
            Self::Quad(quad) => quad.rect.contains(point),
            Self::Mesh(mesh) => mesh.intersects_point(point),
        }
    }
}

impl From<Quad> for Primitive {
    fn from(quad: Quad) -> Self {
        Self::Quad(quad)
    }
}

impl From<Mesh> for Primitive {
    fn from(mesh: Mesh) -> Self {
        Self::Mesh(mesh)
    }
}
