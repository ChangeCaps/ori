use ori_graphics::math::Vec2;

use crate::{AvailableSpace, Context, Length};

#[derive(Clone, Copy, Debug, Default)]
pub struct Size {
    pub min_width: Length,
    pub max_width: Length,
    pub min_height: Length,
    pub max_height: Length,
}

impl Size {
    pub fn new(width: impl Into<Length>, height: impl Into<Length>) -> Self {
        let width = width.into();
        let height = height.into();

        Self {
            min_width: width,
            max_width: width,
            min_height: height,
            max_height: height,
        }
    }

    pub const fn content() -> Self {
        Self {
            min_width: Length::Content(1.0),
            max_width: Length::Content(1.0),
            min_height: Length::Content(1.0),
            max_height: Length::Content(1.0),
        }
    }

    pub const fn parent() -> Self {
        Self {
            min_width: Length::Parent(0.0),
            max_width: Length::Parent(1.0),
            min_height: Length::Parent(0.0),
            max_height: Length::Parent(1.0),
        }
    }

    pub fn set_width(&mut self, width: impl Into<Length>) {
        let width = width.into();
        self.min_width = width;
        self.max_width = width;
    }

    pub fn set_height(&mut self, height: impl Into<Length>) {
        let height = height.into();
        self.min_height = height;
        self.max_height = height;
    }

    pub fn content_space(&self, cx: &Context<'_>, space: AvailableSpace) -> AvailableSpace {
        let min_width = match self.min_width {
            Length::Unit(unit) => unit.get(cx),
            Length::Parent(ratio) => space.max.x * ratio,
            Length::Content(_) => space.min.x,
        };

        let max_width = match self.max_width {
            Length::Unit(unit) => unit.get(cx),
            Length::Parent(ratio) => space.max.x * ratio,
            Length::Content(_) => space.max.x,
        };

        let min_height = match self.min_height {
            Length::Unit(unit) => unit.get(cx),
            Length::Parent(ratio) => space.max.y * ratio,
            Length::Content(_) => space.min.y,
        };

        let max_height = match self.max_height {
            Length::Unit(unit) => unit.get(cx),
            Length::Parent(ratio) => space.max.y * ratio,
            Length::Content(_) => space.max.y,
        };

        let min = Vec2::new(min_width, min_height);
        let max = Vec2::new(max_width, max_height);
        AvailableSpace::new(min, max)
    }

    pub fn resolve(&self, cx: &Context<'_>, content_size: Vec2, space: AvailableSpace) -> Vec2 {
        let min_width = self.min_width.get(cx, content_size.x, space.max.x);
        let min_height = self.min_height.get(cx, content_size.y, space.max.y);
        let max_width = self.max_width.get(cx, content_size.x, space.max.x);
        let max_height = self.max_height.get(cx, content_size.y, space.max.y);

        let width = min_width.max(max_width);
        let height = min_height.max(max_height);

        space.fit(Vec2::new(width, height))
    }
}
