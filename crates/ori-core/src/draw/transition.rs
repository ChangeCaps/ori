#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum TransitionCurve {
    /// Linear transition curve.
    #[default]
    Linear,
    /// Smooth transition curve.
    Smooth,
}

impl TransitionCurve {
    pub fn evaluate(&self, t: f32) -> f32 {
        match self {
            Self::Linear => t,
            Self::Smooth => t * t * (3.0 - 2.0 * t),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Transition {
    pub duration: f32,
    pub curve: TransitionCurve,
}

impl Transition {
    pub fn linear(duration: f32) -> Self {
        Self {
            duration,
            curve: TransitionCurve::Linear,
        }
    }

    pub fn smooth(duration: f32) -> Self {
        Self {
            duration,
            curve: TransitionCurve::Smooth,
        }
    }

    pub fn update(&self, value: &mut f32, active: bool, dt: f32) -> bool {
        let mut updated = false;

        let step = dt / self.duration;

        if active && *value < 1.0 {
            if *value == 0.0 {
                *value += f32::EPSILON;
            } else {
                *value += step;
            }

            updated = true;
        } else if !active && *value > 0.0 {
            if *value == 1.0 {
                *value -= f32::EPSILON;
            } else {
                *value -= step;
            }

            updated = true;
        }

        *value = value.clamp(0.0, 1.0);

        updated
    }

    pub fn get(&self, t: f32) -> f32 {
        self.curve.evaluate(t)
    }
}

impl From<f32> for Transition {
    fn from(duration: f32) -> Self {
        Self::smooth(duration)
    }
}
