pub type AlignSelf = AlignItems;
pub type AlignItem = AlignItems;

/// The alignment of items along the cross axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AlignItems {
    /// Items are packed toward the start of the stack.
    Start,
    /// Items are packed toward the end of the stack.
    End,
    /// Items are packed toward the center of the stack.
    Center,
    /// Items are stretched to fill the stack.
    Stretch,
}

impl AlignItems {
    /// Returns true if the alignment is stretch.
    pub const fn is_stretch(&self) -> bool {
        matches!(self, Self::Stretch)
    }

    /// Aligns an item within the given space.
    pub fn align(self, available: f32, size: f32) -> f32 {
        match self {
            Self::Start => 0.0,
            Self::End => available - size,
            Self::Center => (available - size) / 2.0,
            Self::Stretch => 0.0,
        }
    }
}
