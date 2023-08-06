use std::ops::{Mul, MulAssign};

use glam::{Affine2, Mat2, Vec2};

/// A 2D affine transformation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine {
    /// The translation component of the transformation.
    pub translation: Vec2,
    /// The matrix component of the transformation.
    pub matrix: Mat2,
}

impl Default for Affine {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Affine {
    /// The identity transformation.
    pub const IDENTITY: Self = Self::identity();

    /// Creates a new identity transformation.
    pub const fn identity() -> Self {
        Self {
            translation: Vec2::ZERO,
            matrix: Mat2::IDENTITY,
        }
    }

    /// Creates a new affine transformation.
    pub const fn new(translation: Vec2, matrix: Mat2) -> Self {
        Self {
            translation,
            matrix,
        }
    }

    /// Creates a new affine transformation from a translation.
    pub const fn translation(translation: Vec2) -> Self {
        Self {
            translation,
            matrix: Mat2::IDENTITY,
        }
    }

    /// Creates a new affine transformation from a matrix.
    pub const fn from_matrix(matrix: Mat2) -> Self {
        Self {
            translation: Vec2::ZERO,
            matrix,
        }
    }

    /// Creates a new affine transformation from a rotation.
    pub fn rotation(angle: f32) -> Self {
        Self::from_matrix(Mat2::from_angle(angle))
    }

    /// Creates a new affine transformation from a scale.
    pub const fn scale(scale: Vec2) -> Self {
        Self::from_matrix(Mat2::from_diagonal(scale))
    }

    /// Creates a new affine transformation from a shear.
    pub const fn shear(shear: Vec2) -> Self {
        Self::from_matrix(Mat2::from_cols(Vec2::X, shear))
    }

    /// Computes the inverse of the affine transformation.
    pub fn inverse(self) -> Self {
        Self {
            translation: -self.translation,
            matrix: self.matrix.inverse(),
        }
    }

    /// Rounds the affine transformation's translation to the nearest integers.
    pub fn round(self) -> Self {
        Self {
            translation: self.translation.round(),
            matrix: self.matrix,
        }
    }

    /// Multiplies the affine transformation with a vector.
    pub fn mul_vec2(self, other: Vec2) -> Vec2 {
        self.matrix * other + self.translation
    }

    /// Multiplies the affine transformation with another affine transformation.
    pub fn mul_affine(self, other: Affine) -> Affine {
        Affine {
            translation: self.mul_vec2(other.translation),
            matrix: self.matrix * other.matrix,
        }
    }
}

impl From<Affine2> for Affine {
    fn from(affine: Affine2) -> Self {
        Self {
            translation: affine.translation,
            matrix: affine.matrix2,
        }
    }
}

impl From<Affine> for Affine2 {
    fn from(affine: Affine) -> Self {
        Self {
            translation: affine.translation,
            matrix2: affine.matrix,
        }
    }
}

impl Mul<Vec2> for Affine {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.mul_vec2(rhs)
    }
}

impl Mul<Affine> for Affine {
    type Output = Affine;

    fn mul(self, rhs: Affine) -> Self::Output {
        self.mul_affine(rhs)
    }
}

impl MulAssign<Affine> for Affine {
    fn mul_assign(&mut self, rhs: Affine) {
        *self = self.mul_affine(rhs);
    }
}

impl Mul<Affine> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Affine) -> Self::Output {
        rhs.mul_vec2(self)
    }
}

impl MulAssign<Affine> for Vec2 {
    fn mul_assign(&mut self, rhs: Affine) {
        *self = rhs.mul_vec2(*self);
    }
}
