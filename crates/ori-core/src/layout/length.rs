use crate::{Context, Unit};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Length {
    Unit(Unit),
    Parent(f32),
    Content(f32),
}

impl Default for Length {
    fn default() -> Self {
        Self::Content(1.0)
    }
}

impl From<Unit> for Length {
    fn from(unit: Unit) -> Self {
        Self::Unit(unit)
    }
}

impl Length {
    pub fn get(&self, cx: &Context<'_>, content: f32, parent: f32) -> f32 {
        match self {
            Self::Unit(unit) => unit.get(cx),
            Self::Parent(ratio) => parent * ratio,
            Self::Content(ratio) => content * ratio,
        }
    }
}
