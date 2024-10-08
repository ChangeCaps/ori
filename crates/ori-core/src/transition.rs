//! Transition utilities.

/// Create a linear transition with the given `duration`.
pub fn linear(duration: f32) -> Transition {
    Transition::linear(duration)
}

/// Create an ease transition with the given `duration`.
pub fn ease(duration: f32) -> Transition {
    Transition::ease(duration)
}

/// A transition easing curve.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Easing {
    /// A linear transition curve.
    #[default]
    Linear,

    /// An ease transition curve.
    Ease,
}

impl Easing {
    /// Evaluate the easing at `t` where `0 <= t <= 1`.
    pub fn evaluate(self, t: f32) -> f32 {
        match self {
            Easing::Linear => t,
            Easing::Ease => t * t * (3.0 - 2.0 * t),
        }
    }
}

/// A transition.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transition {
    /// The duration of the transition.
    pub duration: f32,
    /// The easing curve.
    pub easing: Easing,
}

impl Default for Transition {
    fn default() -> Self {
        Self::ease(0.2)
    }
}

impl Transition {
    /// Create a linear transition with the given `duration`.
    pub fn linear(duration: f32) -> Self {
        Self {
            duration,
            easing: Easing::Linear,
        }
    }

    /// Create an ease transition with the given `duration`.
    pub fn ease(duration: f32) -> Self {
        Self {
            duration,
            easing: Easing::Ease,
        }
    }

    /// Step the transition.
    pub fn step(&self, t: &mut f32, on: bool, dt: f32) -> bool {
        let sign = if on { 1.0 } else { -1.0 };
        let step = sign * dt / self.duration;
        let to = if on { 1.0 } else { 0.0 };

        if *t == to {
            return false;
        }

        *t += step;
        *t = t.clamp(0.0, 1.0);

        true
    }

    /// Check if the transition is complete.
    pub fn complete(&self, t: f32, on: bool) -> bool {
        (t == 0.0 && !on) || (t == 1.0 && on)
    }

    /// Evaluate the transition curve at `t`.
    ///
    /// The returned value is how _on_ the transition is at `t`.
    /// This is a range from 0.0 to 1.0.
    pub fn get(&self, t: f32) -> f32 {
        self.easing.evaluate(t)
    }
}
