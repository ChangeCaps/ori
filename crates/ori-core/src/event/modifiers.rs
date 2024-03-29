/// Modifiers for keyboard and pointer events.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Modifiers {
    /// Whether the shift key is held down.
    pub shift: bool,
    /// Whether the control key is held down.
    pub ctrl: bool,
    /// Whether the alt key is held down.
    pub alt: bool,
    /// Whether the meta key is held down.
    pub meta: bool,
}

impl Modifiers {
    /// Check if any modifiers are held down.
    pub fn any(&self) -> bool {
        self.shift || self.ctrl || self.alt || self.meta
    }
}
