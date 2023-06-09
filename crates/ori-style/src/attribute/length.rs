use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    mem,
    ops::Range,
};

pub use Length::*;

/// A length. (eg. 10px, 10pt, 10%)
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Length {
    /// Unit of measurement in pixels. (eg. 10px)
    ///
    /// This is the default unit.
    Px(f32),
    /// Unit of measurement in points. (eg. 10pt)
    ///
    /// 1pt = 1/72 inch
    Pt(f32),
    /// Unit of measurement in percent. (eg. 10%)
    ///
    /// The percent is context specific, and is often relative
    /// to the parent's size, but doesn't have to be.
    Pc(f32),
    /// Unit of measurement in viewport width. (eg. 10vw)
    Vw(f32),
    /// Unit of measurement in viewport height. (eg. 10vh)
    Vh(f32),
    /// Unit of measurement in em. (eg. 10em)
    ///
    /// 1em = the font size of the root.
    /// 1em = 16px by default.
    Em(f32),
}

impl Eq for Length {}

impl Hash for Length {
    fn hash<H: Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state);

        match self {
            Px(value) => value.to_bits().hash(state),
            Pt(value) => value.to_bits().hash(state),
            Pc(value) => value.to_bits().hash(state),
            Vw(value) => value.to_bits().hash(state),
            Vh(value) => value.to_bits().hash(state),
            Em(value) => value.to_bits().hash(state),
        }
    }
}

impl Default for Length {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Length {
    pub const ZERO: Self = Px(0.0);

    pub fn pixels(
        self,
        range: Range<f32>,
        scale: f32,
        window_width: f32,
        window_height: f32,
    ) -> f32 {
        match self {
            Px(value) => value,
            Pt(value) => value * 96.0 / 72.0 * scale,
            Pc(value) => range.start + (range.end - range.start) * value / 100.0,
            Vw(value) => value * window_width / 100.0,
            Vh(value) => value * window_height / 100.0,
            Em(value) => value * 16.0 * scale,
        }
    }

    pub fn as_f32(self) -> f32 {
        match self {
            Px(value) => value,
            Pt(value) => value,
            Pc(value) => value,
            Vw(value) => value,
            Vh(value) => value,
            Em(value) => value,
        }
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Px(value) => write!(f, "{}px", value),
            Pt(value) => write!(f, "{}pt", value),
            Pc(value) => write!(f, "{}%", value),
            Vw(value) => write!(f, "{}vw", value),
            Vh(value) => write!(f, "{}vh", value),
            Em(value) => write!(f, "{}em", value),
        }
    }
}
