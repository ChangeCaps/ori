use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use parking_lot::MutexGuard;

use crate::{AnyElement, Build, Element};

/// A guard for [`Build::prop`].
///
/// See [`Node::prop`](crate::Node::prop).
pub struct PropGuard<'a, B> {
    properties: MutexGuard<'a, Box<dyn AnyElement>>,
    marker: PhantomData<B>,
}

impl<'a, B> PropGuard<'a, B> {
    /// Create a new property guard.
    ///
    /// # Safety
    /// - `properties` must be of type `B`.
    pub unsafe fn new(properties: MutexGuard<'a, Box<dyn AnyElement>>) -> Self {
        Self {
            properties,
            marker: PhantomData,
        }
    }
}

impl<'a, B: Element + Build + 'a> Deref for PropGuard<'a, B> {
    type Target = B::Properties;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `self.properties` is of type `B`.
        let element: &B = unsafe { self.properties.downcast_ref_unchecked() };
        element.prop_ref()
    }
}

impl<'a, B: Element + Build + 'a> DerefMut for PropGuard<'a, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `self.properties` is of type `B`.
        let element: &mut B = unsafe { self.properties.downcast_mut_unchecked() };
        element.prop()
    }
}

/// A guard for [`Build::on`].
///
/// See [`Node::on`](crate::Node::on).
pub struct OnGuard<'a, B> {
    events: MutexGuard<'a, Box<dyn AnyElement>>,
    marker: PhantomData<B>,
}

impl<'a, B: Element + Build + 'a> Deref for OnGuard<'a, B> {
    type Target = B::Events;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `self.events` is of type `B`.
        let element: &B = unsafe { self.events.downcast_ref_unchecked() };
        element.on_ref()
    }
}

impl<'a, B: Element + Build + 'a> DerefMut for OnGuard<'a, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `self.events` is of type `B`.
        let element: &mut B = unsafe { self.events.downcast_mut_unchecked() };
        element.on()
    }
}

impl<'a, B: Element + Build + 'a> OnGuard<'a, B> {
    /// Create a new event guard.
    ///
    /// # Safety
    /// - `events` must be of type `B`.
    pub unsafe fn new(events: MutexGuard<'a, Box<dyn AnyElement>>) -> Self {
        Self {
            events,
            marker: PhantomData,
        }
    }
}

/// A guard for [`Build::bind`].
///
/// See [`Node::bind`](crate::Node::bind).
pub struct BindGuard<'a, B> {
    bindings: MutexGuard<'a, Box<dyn AnyElement>>,
    marker: PhantomData<B>,
}

impl<'a, B: Element + Build + 'a> Deref for BindGuard<'a, B> {
    type Target = B::Bindings;

    fn deref(&self) -> &Self::Target {
        // SAFETY: `self.bindings` is of type `B`.
        let element: &B = unsafe { self.bindings.downcast_ref_unchecked() };
        element.bind_ref()
    }
}

impl<'a, B: Element + Build + 'a> DerefMut for BindGuard<'a, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `self.bindings` is of type `B`.
        let element: &mut B = unsafe { self.bindings.downcast_mut_unchecked() };
        element.bind()
    }
}

impl<'a, B: Element + Build + 'a> BindGuard<'a, B> {
    /// Create a new binding guard.
    ///
    /// # Safety
    /// - `bindings` must be of type `B`.
    pub unsafe fn new(bindings: MutexGuard<'a, Box<dyn AnyElement>>) -> Self {
        Self {
            bindings,
            marker: PhantomData,
        }
    }
}
