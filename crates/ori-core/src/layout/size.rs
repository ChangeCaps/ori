use ori_graphics::math::Vec2;

use crate::{AvailableSpace, Context, Length};

#[derive(Clone, Copy, Debug, Default)]
pub struct Size {
    pub width: Length,
    pub height: Length,
}

impl Size {
    pub fn new(width: impl Into<Length>, height: impl Into<Length>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }

    pub const fn content() -> Self {
        Self {
            width: Length::Content(1.0),
            height: Length::Content(1.0),
        }
    }

    pub const fn parent() -> Self {
        Self {
            width: Length::Parent(1.0),
            height: Length::Parent(1.0),
        }
    }

    pub fn get(&self, cx: &Context<'_>, content: Vec2, space: AvailableSpace) -> Vec2 {
        let width = self.width.get(cx, content.x, space.max.x);
        let height = self.height.get(cx, content.y, space.max.y);

        space.fit(Vec2::new(width, height))
    }

    pub fn content_space(&self, cx: &Context<'_>, space: AvailableSpace) -> AvailableSpace {
        let max_width = match self.width {
            Length::Unit(unit) => unit.get(cx),
            Length::Parent(ratio) => space.max.x * ratio,
            Length::Content(_) => space.max.x,
        };

        let max_height = match self.height {
            Length::Unit(unit) => unit.get(cx),
            Length::Parent(ratio) => space.max.y * ratio,
            Length::Content(_) => space.max.y,
        };

        let max = Vec2::new(max_width, max_height);
        AvailableSpace::new(space.min.min(max), max)
    }
}
