use std::{
    ops::{Deref, DerefMut},
    sync::OnceLock,
};

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::Emitter;

struct AtomRefInner<T: 'static> {
    emitter: Emitter,
    value: RwLock<T>,
}

impl<T: 'static> AtomRefInner<T> {
    fn new(value: T) -> Self {
        Self {
            emitter: Emitter::new(),
            value: RwLock::new(value),
        }
    }
}

/// A thread-safe, reactive reference to a value.
///
/// This is similar to [`Atom`](crate::Atom), but should be used for types that
/// don't implement [`Clone`].
///
/// # Example
/// ```
/// # use ori_reactive::prelude::*;
/// // this is a static AtomRef, created with the `atom!` macro
/// static COUNTER: AtomRef<i32> = atom!(ref 0);
///
/// // we can read the value with `read`
/// assert_eq!(*COUNTER.read(), 0);
///
/// // and write the value with `write`
/// *COUNTER.write() += 1;
///
/// // and read it again
/// assert_eq!(*COUNTER.read(), 1);
/// ```
pub struct AtomRef<T: 'static> {
    inner: OnceLock<AtomRefInner<T>>,
    init: fn() -> T,
}

impl<T> AtomRef<T> {
    /// Creates a new [`AtomRef`] with the given value.
    pub fn new(value: T) -> Self {
        Self {
            inner: OnceLock::from(AtomRefInner::new(value)),
            init: || unreachable!(),
        }
    }

    /// Creates a new [`AtomRef`] with the given initializer.
    ///
    /// See [`atom!`](crate::atom!) for more information.
    pub const fn init(init: fn() -> T) -> Self {
        Self {
            inner: OnceLock::new(),
            init,
        }
    }
}

impl<T: Send + Sync> AtomRef<T> {
    fn inner(&self) -> &AtomRefInner<T> {
        self.inner.get_or_init(|| AtomRefInner::new((self.init)()))
    }

    /// Returns a reference to the [`RwLock`] that contains the value.
    pub fn lock(&self) -> &RwLock<T> {
        &self.inner().value
    }

    /// Returns a reference to the [`Emitter`] that is triggered when
    /// the value is modified.
    pub fn emitter(&self) -> &Emitter {
        &self.inner().emitter
    }

    /// Emits the [`Emitter`].
    pub fn emit(&self) {
        self.emitter().emit(&());
    }

    /// Tracks the [`Emitter`] for the current effect.
    pub fn track(&self) {
        crate::effect::track_callback(self.emitter().downgrade());
    }

    /// Returns a reference to the value.
    pub fn read(&self) -> AtomReadGuard<'_, T> {
        AtomReadGuard {
            guard: self.lock().read(),
            emitter: self.emitter(),
        }
    }

    /// Returns a mutable reference to the value.
    pub fn write(&self) -> AtomWriteGuard<'_, T> {
        AtomWriteGuard {
            guard: Some(self.lock().write()),
            emitter: self.emitter(),
        }
    }
}

/// A guard that tracks the [`Emitter`] when the value is read.
pub struct AtomReadGuard<'a, T> {
    guard: RwLockReadGuard<'a, T>,
    emitter: &'a Emitter,
}

impl<'a, T> Deref for AtomReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        crate::effect::track_callback(self.emitter.downgrade());
        &self.guard
    }
}

/// A guard that tracks the [`Emitter`] when read, and emits it when
/// dropped.
pub struct AtomWriteGuard<'a, T> {
    guard: Option<RwLockWriteGuard<'a, T>>,
    emitter: &'a Emitter,
}

impl<'a, T> Deref for AtomWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        crate::effect::track_callback(self.emitter.downgrade());
        self.guard.as_ref().unwrap()
    }
}

impl<'a, T> DerefMut for AtomWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        crate::effect::track_callback(self.emitter.downgrade());
        self.guard.as_mut().unwrap()
    }
}

impl<'a, T> Drop for AtomWriteGuard<'a, T> {
    fn drop(&mut self) {
        self.guard.take();
        self.emitter.clear_and_emit(&());
    }
}
