use std::ops::Add;

use glam::Vec2;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AvailableSpace {
    pub min: Vec2,
    pub max: Vec2,
}

impl AvailableSpace {
    /// Create a new available space.
    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Create a new available space with the same min and max.
    pub const fn uniform(size: Vec2) -> Self {
        Self::new(size, size)
    }

    /// Apply padding to the available space, this will reduce the min and max
    /// by [`Padding::size()`].
    pub fn pad(self, padding: Padding) -> Self {
        let min = self.min - padding.size();
        let max = self.max - padding.size();
        Self::new(min.max(Vec2::ZERO), max.max(Vec2::ZERO))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl From<f32> for Padding {
    fn from(value: f32) -> Self {
        Self::uniform(value)
    }
}

impl Padding {
    /// Create a new padding.
    pub const fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Create a new padding with the same left, right, top and bottom.
    pub const fn uniform(size: f32) -> Self {
        Self::new(size, size, size, size)
    }

    /// Get the size of the padding, i.e. `left + right` and `top + bottom`.
    pub fn size(self) -> Vec2 {
        Vec2::new(self.left + self.right, self.top + self.bottom)
    }

    /// Get the translation of the padding, i.e. `left` and `top`.
    pub fn translation(self) -> Vec2 {
        Vec2::new(self.left, self.top)
    }
}

impl Add<Padding> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Padding) -> Self::Output {
        self + rhs.size()
    }
}
