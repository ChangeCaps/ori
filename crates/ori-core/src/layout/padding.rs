use ori_graphics::math::Vec2;

use crate::{Context, Unit};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Padding {
    pub left: Unit,
    pub right: Unit,
    pub top: Unit,
    pub bottom: Unit,
}

impl From<Unit> for Padding {
    fn from(value: Unit) -> Self {
        Self::all(value)
    }
}

impl Padding {
    /// Create a new padding.
    pub const fn new(left: Unit, right: Unit, top: Unit, bottom: Unit) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Create a new padding with the same left, right, top and bottom.
    pub const fn all(size: Unit) -> Self {
        Self::new(size, size, size, size)
    }

    /// Get the size of the padding, i.e. `left + right` and `top + bottom`.
    pub fn size(self, cx: &Context<'_>) -> Vec2 {
        Vec2::new(
            self.left.get(cx) + self.right.get(cx),
            self.top.get(cx) + self.bottom.get(cx),
        )
    }

    /// Get the translation of the padding, i.e. `left` and `top`.
    pub fn offset(self, cx: &Context<'_>) -> Vec2 {
        Vec2::new(self.left.get(cx), self.top.get(cx))
    }
}
