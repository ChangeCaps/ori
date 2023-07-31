/// An event that is sent to an element before it is laid out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PrepareLayoutEvent;

/// An event that forces a layout of an element.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ForceLayoutEvent;
