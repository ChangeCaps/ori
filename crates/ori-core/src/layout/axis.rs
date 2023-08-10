use glam::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    Row,
    Column,
}

impl Axis {
    pub const fn cross(self) -> Self {
        match self {
            Self::Row => Self::Column,
            Self::Column => Self::Row,
        }
    }

    pub fn major(self, vec: Vec2) -> f32 {
        match self {
            Self::Row => vec.x,
            Self::Column => vec.y,
        }
    }

    pub fn minor(self, vec: Vec2) -> f32 {
        match self {
            Self::Row => vec.y,
            Self::Column => vec.x,
        }
    }

    pub fn pack(self, major: f32, minor: f32) -> Vec2 {
        match self {
            Self::Row => Vec2::new(major, minor),
            Self::Column => Vec2::new(minor, major),
        }
    }

    pub fn unpack(self, vec: Vec2) -> (f32, f32) {
        match self {
            Self::Row => (vec.x, vec.y),
            Self::Column => (vec.y, vec.x),
        }
    }
}
