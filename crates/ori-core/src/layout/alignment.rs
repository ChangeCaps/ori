use std::ops::Mul;

use ori_graphics::math::Vec2;

/// The alignment of content in a view.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Alignment(Vec2);

impl Alignment {
    /// Create a new alignment.
    pub const fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    /// Create a new alignment at the center.
    pub const fn center() -> Self {
        Self::new(0.5, 0.5)
    }

    /// Create a new alignment at the top left.
    pub const fn top_left() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Create a new alignment at the top.
    pub const fn top() -> Self {
        Self::new(0.5, 0.0)
    }

    /// Create a new alignment at the top right.
    pub const fn top_right() -> Self {
        Self::new(1.0, 0.0)
    }

    /// Create a new alignment at the left.
    pub const fn left() -> Self {
        Self::new(0.0, 0.5)
    }

    /// Create a new alignment at the right.
    pub const fn right() -> Self {
        Self::new(1.0, 0.5)
    }

    /// Create a new alignment at the bottom left.
    pub const fn bottom_left() -> Self {
        Self::new(0.0, 1.0)
    }

    /// Create a new alignment at the bottom.
    pub const fn bottom() -> Self {
        Self::new(0.5, 1.0)
    }

    /// Create a new alignment at the bottom right.
    pub const fn bottom_right() -> Self {
        Self::new(1.0, 1.0)
    }

    /// Align the content in the given size.
    ///
    /// This will return the offset that should be applied to the content to
    /// align it.
    pub fn align(self, content: Vec2, size: Vec2) -> Vec2 {
        (size - content) * self
    }
}

impl From<Vec2> for Alignment {
    fn from(vec: Vec2) -> Self {
        Self(vec)
    }
}

impl From<(f32, f32)> for Alignment {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

impl Mul<Alignment> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Alignment) -> Self::Output {
        self * rhs.0
    }
}

impl Mul<Vec2> for Alignment {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.0 * rhs
    }
}
