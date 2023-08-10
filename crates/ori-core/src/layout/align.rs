pub type AlignSelf = AlignItems;
pub type AlignItem = AlignItems;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Stretch,
}

impl AlignItems {
    pub const fn is_stretch(&self) -> bool {
        matches!(self, Self::Stretch)
    }

    pub fn align(self, available: f32, size: f32) -> f32 {
        match self {
            Self::Start => 0.0,
            Self::End => available - size,
            Self::Center => (available - size) / 2.0,
            Self::Stretch => 0.0,
        }
    }
}
