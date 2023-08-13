use std::ops::{Mul, MulAssign};

use crate::{Context, Unit};

#[derive(Clone, Copy, Debug, Default)]
pub struct BorderWidth {
    pub top: Unit,
    pub right: Unit,
    pub bottom: Unit,
    pub left: Unit,
}

impl BorderWidth {
    pub const ZERO: Self = Self::all(Unit::ZERO);

    pub const fn new(top: Unit, right: Unit, bottom: Unit, left: Unit) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub const fn all(width: Unit) -> Self {
        Self::new(width, width, width, width)
    }

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

#[derive(Clone, Copy, Debug, Default)]
pub struct BorderRadius {
    pub top_left: Unit,
    pub top_right: Unit,
    pub bottom_right: Unit,
    pub bottom_left: Unit,
}

impl BorderRadius {
    pub const ZERO: Self = Self::all(Unit::ZERO);

    pub const fn new(
        top_left: Unit,
        top_right: Unit,
        bottom_right: Unit,
        bottom_left: Unit,
    ) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }

    pub const fn all(radius: Unit) -> Self {
        Self::new(radius, radius, radius, radius)
    }

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
