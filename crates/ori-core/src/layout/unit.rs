use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    mem,
};

use ori_graphics::math::Vec2;
pub use Unit::*;

use crate::Context;

/// A length. (eg. 10px, 10pt, 10%)
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Unit {
    /// Unit of measurement in pixels. (eg. 10px)
    ///
    /// This is the default unit.
    Px(f32),
    /// Unit of measurement in points. (eg. 10pt)
    ///
    /// 1pt = 1/72 inch
    Pt(f32),
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

impl Eq for Unit {}

impl Hash for Unit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state);

        match self {
            Px(value) => value.to_bits().hash(state),
            Pt(value) => value.to_bits().hash(state),
            Vw(value) => value.to_bits().hash(state),
            Vh(value) => value.to_bits().hash(state),
            Em(value) => value.to_bits().hash(state),
        }
    }
}

impl From<f32> for Unit {
    fn from(value: f32) -> Self {
        Self::Px(value)
    }
}

impl Default for Unit {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Unit {
    pub const ZERO: Self = Px(0.0);
    pub const INFINITY: Self = Px(f32::INFINITY);

    pub fn resolve(self, scale: f32, window_size: Vec2) -> f32 {
        match self {
            Px(value) => value,
            Pt(value) => value * 96.0 / 72.0 * scale,
            Vw(value) => value * window_size.x / 100.0,
            Vh(value) => value * window_size.y / 100.0,
            Em(value) => value * 16.0 * scale,
        }
    }

    pub fn get(self, cx: &Context<'_>) -> f32 {
        cx.unit(self)
    }

    pub fn as_f32(self) -> f32 {
        match self {
            Px(value) => value,
            Pt(value) => value,
            Vw(value) => value,
            Vh(value) => value,
            Em(value) => value,
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Px(value) => write!(f, "{}px", value),
            Pt(value) => write!(f, "{}pt", value),
            Vw(value) => write!(f, "{}vw", value),
            Vh(value) => write!(f, "{}vh", value),
            Em(value) => write!(f, "{}em", value),
        }
    }
}
