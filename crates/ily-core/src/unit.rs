use std::{
    fmt::Display,
    ops::{Add, Mul},
};

macro_rules! unit {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
        pub struct $name(pub f32);

        impl From<$name> for f32 {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<f32> for $name {
            fn from(value: f32) -> Self {
                Self(value)
            }
        }

        impl From<$name> for Length {
            fn from(value: $name) -> Self {
                Self::$name(value)
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl std::ops::AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl std::ops::SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl std::ops::Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.0 * rhs.0)
            }
        }

        impl std::ops::MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
            }
        }

        impl std::ops::Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self(self.0 / rhs.0)
            }
        }

        impl std::ops::DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0;
            }
        }

        impl std::ops::Neg for $name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl std::ops::Rem for $name {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self::Output {
                Self(self.0 % rhs.0)
            }
        }

        impl std::ops::RemAssign for $name {
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0;
            }
        }

        impl std::ops::Deref for $name {
            type Target = f32;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl AsRef<f32> for $name {
            fn as_ref(&self) -> &f32 {
                &self.0
            }
        }

        impl AsMut<f32> for $name {
            fn as_mut(&mut self) -> &mut f32 {
                &mut self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}{}", self.0, stringify!($name).to_lowercase())
            }
        }
    };
    (
        $(
            $(#[$meta:meta])*
            $name:ident
        ),*
        $(,)?
    ) => {
        $(unit!($(#[$meta])* $name);)*
    };
}

unit! {
    /// A unit of length in pixels.
    Px,
    /// A unit of length in points.
    Pt,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Length {
    Px(Px),
    Pt(Pt),
}

impl Default for Length {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Length {
    pub const ZERO: Self = Self::Px(Px(0.0));

    pub const fn px(value: f32) -> Self {
        Self::Px(Px(value))
    }

    pub const fn pt(value: f32) -> Self {
        Self::Pt(Pt(value))
    }

    pub fn pixels(&self) -> f32 {
        match self {
            Self::Px(value) => value.0,
            Self::Pt(value) => value.0 * 96.0 / 72.0,
        }
    }
}

impl From<f32> for Length {
    fn from(value: f32) -> Self {
        Self::Px(Px(value))
    }
}

impl Add for Length {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::px(self.pixels() + rhs.pixels())
    }
}

impl Mul<f32> for Length {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Self::Px(value) => Self::px(*value * rhs),
            Self::Pt(value) => Self::pt(*value * rhs),
        }
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Px(value) => write!(f, "{}", value),
            Self::Pt(value) => write!(f, "{}", value),
        }
    }
}
