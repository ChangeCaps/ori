use ori_graphics::math::Vec2;

use crate::{Context, Unit};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Padding {
    pub top: Unit,
    pub right: Unit,
    pub bottom: Unit,
    pub left: Unit,
}

impl Padding {
    /// Create a new padding.
    pub const fn new(top: Unit, right: Unit, bottom: Unit, left: Unit) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
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

impl From<(Unit, Unit, Unit, Unit)> for Padding {
    fn from((top, right, bottom, left): (Unit, Unit, Unit, Unit)) -> Self {
        Self::new(top, right, bottom, left)
    }
}

impl From<(Unit, Unit)> for Padding {
    fn from((horizontal, vertical): (Unit, Unit)) -> Self {
        Self::new(vertical, horizontal, vertical, horizontal)
    }
}

impl From<Unit> for Padding {
    fn from(size: Unit) -> Self {
        Self::all(size)
    }
}
