use crate::{AnyElement, Element, NodeElement};

/// An error that occurs when downcasting an [`NodeElement`] fails.
#[derive(Clone, Copy, Debug, Default)]
pub struct ElementDowncastError;

/// A trait for downcasting a [`NodeElement`] to a specific [`NodeElement`].
pub trait DowncastElement<T: NodeElement> {
    /// Downcast the [`Element`](crate::Element) to `&T`.
    fn downcast_ref(&self) -> Option<&T>;
    /// Downcast the [`Element`](crate::Element) to `&mut T`.
    fn downcast_mut(&mut self) -> Option<&mut T>;
}

impl<T: Element> DowncastElement<T> for T {
    fn downcast_ref(&self) -> Option<&T> {
        Some(self)
    }

    fn downcast_mut(&mut self) -> Option<&mut T> {
        Some(self)
    }
}

impl<T: Element> DowncastElement<T> for Box<dyn AnyElement> {
    fn downcast_ref(&self) -> Option<&T> {
        self.as_ref().downcast_ref()
    }

    fn downcast_mut(&mut self) -> Option<&mut T> {
        self.as_mut().downcast_mut()
    }
}

impl DowncastElement<Box<dyn AnyElement>> for Box<dyn AnyElement> {
    fn downcast_ref(&self) -> Option<&Box<dyn AnyElement>> {
        Some(self)
    }

    fn downcast_mut(&mut self) -> Option<&mut Box<dyn AnyElement>> {
        Some(self)
    }
}
