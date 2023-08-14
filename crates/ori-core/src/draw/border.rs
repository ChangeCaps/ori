use std::ops::{Mul, MulAssign};

use crate::{Context, Unit};

/// Width of a border on a box.
#[derive(Clone, Copy, Debug, Default)]
pub struct BorderWidth {
    /// Width of the top border.
    pub top: Unit,
    /// Width of the right border.
    pub right: Unit,
    /// Width of the bottom border.
    pub bottom: Unit,
    /// Width of the left border.
    pub left: Unit,
}

impl BorderWidth {
    /// No border.
    pub const ZERO: Self = Self {
        top: Unit::ZERO,
        right: Unit::ZERO,
        bottom: Unit::ZERO,
        left: Unit::ZERO,
    };

    /// Create a new border width.
    pub fn new(
        top: impl Into<Unit>,
        right: impl Into<Unit>,
        bottom: impl Into<Unit>,
        left: impl Into<Unit>,
    ) -> Self {
        Self {
            top: top.into(),
            right: right.into(),
            bottom: bottom.into(),
            left: left.into(),
        }
    }

    /// Create a new border width with the same left, right, top and bottom.
    pub fn all(width: impl Into<Unit>) -> Self {
        let width = width.into();
        Self::new(width, width, width, width)
    }

    /// Get the border width as an array of four pixel values.
    pub fn get(self, cx: &Context<'_>) -> [f32; 4] {
        [
            self.top.get(cx),
            self.right.get(cx),
            self.bottom.get(cx),
            self.left.get(cx),
        ]
    }
}

impl Mul<f32> for BorderWidth {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            self.top * rhs,
            self.right * rhs,
            self.bottom * rhs,
            self.left * rhs,
        )
    }
}

impl MulAssign<f32> for BorderWidth {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

/// Radius of a border on a box.
#[derive(Clone, Copy, Debug, Default)]
pub struct BorderRadius {
    /// Radius of the top left corner.
    pub top_left: Unit,
    /// Radius of the top right corner.
    pub top_right: Unit,
    /// Radius of the bottom right corner.
    pub bottom_right: Unit,
    /// Radius of the bottom left corner.
    pub bottom_left: Unit,
}

impl BorderRadius {
    /// No border radius.
    pub const ZERO: Self = Self {
        top_left: Unit::ZERO,
        top_right: Unit::ZERO,
        bottom_right: Unit::ZERO,
        bottom_left: Unit::ZERO,
    };

    /// Create a new border radius.
    pub fn new(
        top_left: impl Into<Unit>,
        top_right: impl Into<Unit>,
        bottom_right: impl Into<Unit>,
        bottom_left: impl Into<Unit>,
    ) -> Self {
        Self {
            top_left: top_left.into(),
            top_right: top_right.into(),
            bottom_right: bottom_right.into(),
            bottom_left: bottom_left.into(),
        }
    }

    /// Create a new border radius with the same left, right, top and bottom.
    pub fn all(radius: impl Into<Unit>) -> Self {
        let radius = radius.into();
        Self::new(radius, radius, radius, radius)
    }

    /// Get the border radius as an array of four pixel values.
    pub fn get(self, cx: &Context<'_>) -> [f32; 4] {
        [
            self.top_left.get(cx),
            self.top_right.get(cx),
            self.bottom_right.get(cx),
            self.bottom_left.get(cx),
        ]
    }
}

impl Mul<f32> for BorderRadius {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            self.top_left * rhs,
            self.top_right * rhs,
            self.bottom_right * rhs,
            self.bottom_left * rhs,
        )
    }
}

impl MulAssign<f32> for BorderRadius {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}
