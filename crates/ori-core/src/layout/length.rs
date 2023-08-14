use crate::{Context, Unit};

/// A length that can be used to specify the size of a widget.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Length {
    /// A length specified in a specific [`Unit`].
    Unit(Unit),
    /// A length specified as a ratio of the parent size.
    Parent(f32),
    /// A length specified as a ratio of the content size.
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
    /// Resolves the length in pixels.
    pub fn get(&self, cx: &Context<'_>, content: f32, parent: f32) -> f32 {
        match self {
            Self::Unit(unit) => unit.get(cx),
            Self::Parent(ratio) => parent * ratio,
            Self::Content(ratio) => content * ratio,
        }
    }
}
