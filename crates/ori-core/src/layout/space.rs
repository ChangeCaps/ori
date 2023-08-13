use ori_graphics::math::Vec2;

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

    pub fn fit(self, size: Vec2) -> Vec2 {
        size.clamp(self.min, self.max)
    }

    pub fn shrink(self, size: Vec2) -> Self {
        Self::new(
            Vec2::max(self.min - size, Vec2::ZERO),
            Vec2::max(self.max - size, Vec2::ZERO),
        )
    }

    pub fn stretch_width(mut self) -> Self {
        self.min.x = self.max.x;
        self
    }

    pub fn stretch_height(mut self) -> Self {
        self.min.y = self.max.y;
        self
    }

    pub fn stretch(mut self) -> Self {
        self.min = self.max;
        self
    }
}
