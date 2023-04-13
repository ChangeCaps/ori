use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    panic::Location,
    sync::{Arc, Mutex},
};

use crate::CallbackEmitter;

/// A read-only [`Signal`].
pub struct ReadSignal<T: ?Sized> {
    value: Mutex<Arc<T>>,
    emitter: CallbackEmitter,
}

impl<T> ReadSignal<T> {
    /// Creates a new [`ReadSignal`] from a value.
    pub fn new(value: T) -> Self {
        Self::new_arc(Arc::new(value))
    }
}

impl<T: ?Sized> ReadSignal<T> {
    /// Creates a new [`ReadSignal`] from an [`Arc`].
    pub fn new_arc(value: Arc<T>) -> Self {
        Self {
            value: Mutex::new(value),
            emitter: CallbackEmitter::new(),
        }
    }

    /// Gets the [`CallbackEmitter`] for this [`ReadSignal`].
    pub fn emitter(&self) -> &CallbackEmitter {
        &self.emitter
    }

    /// Tracks `self` in the currently running `effect`.
    pub fn track(&self) {
        self.emitter.track();
    }

    /// Gets the current value of `self`.
    ///
    /// This will track `self` in the currently running `effect`.
    pub fn get(&self) -> Arc<T> {
        self.emitter.track();
        self.get_untracked()
    }

    /// Gets the current value of `self` without tracking it.
    pub fn get_untracked(&self) -> Arc<T> {
        self.value.lock().unwrap().clone()
    }
}

impl<T: Clone> ReadSignal<T> {
    /// Returns a clone of the current value of `self`.
    ///
    /// This will track `self` in the currently running `effect`.
    pub fn cloned(&self) -> T {
        self.get().as_ref().clone()
    }

    /// Returns a clone of the current value of `self` without tracking it.
    pub fn cloned_untracked(&self) -> T {
        self.get_untracked().as_ref().clone()
    }
}

/// A [`Signal`] that can be written to.
///
/// This is a wrapper around [`ReadSignal`].
///
/// Signals are used to store state that can be read from and written to.
/// Using [`Signal::get`] and [`Signal::set`]. Getting the value of a signal
/// will track the signal in the currently running `effect`, and setting the
/// value of a signal will trigger all the callbacks and effects, that are subscribed to
/// the signal.
///
/// # Example
/// ```
/// # use ily_core::*;
/// # Scope::immediate(|cx| {
/// // create a new signal
/// let signal = cx.signal(0);
///
/// // create a new effect
/// cx.effect(|| {
///     // this will be called when it's created
///     // and every time the signal is set
///     println!("signal value: {}", signal.get());
/// });
///
/// // set the signal to 1
/// // this will trigger the effect
/// signal.set(1);
/// # });
/// ```
pub struct Signal<T: ?Sized>(ReadSignal<T>);

impl<T> Signal<T> {
    /// Creates a new [`Signal`] from a value.
    pub fn new(value: T) -> Self {
        Self(ReadSignal::new(value))
    }

    /// Sets the value of `self`.
    #[track_caller]
    pub fn set(&self, value: T) {
        self.set_arc(Arc::new(value));
    }

    /// Sets the value of `self` without triggering the callbacks.
    pub fn set_silent(&self, value: T) {
        self.set_arc_silent(Arc::new(value));
    }
}

impl<T: ?Sized> Signal<T> {
    /// Creates a new [`Signal`] from an [`Arc`].
    pub fn new_arc(value: Arc<T>) -> Self {
        Self(ReadSignal::new_arc(value))
    }

    /// Sets the value of `self` to an [`Arc`].
    #[track_caller]
    pub fn set_arc(&self, value: Arc<T>) {
        self.set_arc_silent(value.clone());
        self.emit();
    }

    /// Sets the value of `self` to an [`Arc`] without triggering the callbacks.
    pub fn set_arc_silent(&self, value: Arc<T>) {
        *self.value.lock().unwrap() = value;
    }

    /// Emits the [`CallbackEmitter`] for this [`Signal`].
    #[track_caller]
    pub fn emit(&self) {
        let location = Location::caller();
        tracing::trace!("emitting signal at {}", location);

        self.emitter.emit(&());
    }
}

impl<T: ?Sized> Deref for Signal<T> {
    type Target = ReadSignal<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Modify<'a, T> {
    value: Option<T>,
    signal: &'a Signal<T>,
}

impl<'a, T> Deref for Modify<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap()
    }
}

impl<'a, T> DerefMut for Modify<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().unwrap()
    }
}

/// When the [`Modify`] is dropped, update the [`Signal`].
impl<'a, T> Drop for Modify<'a, T> {
    fn drop(&mut self) {
        self.signal.set(self.value.take().unwrap());
    }
}

impl<T: Clone> Signal<T> {
    /// Returns a [`Modify`] that can be used to modify the value of the [`Signal`].
    /// When the [`Modify`] is dropped, the [`Signal`] will be updated.
    pub fn modify(&self) -> Modify<'_, T> {
        Modify {
            value: Some(self.get().as_ref().clone()),
            signal: self,
        }
    }
}

/// A [`Signal`] that can be cloned.
pub struct SharedSignal<T: ?Sized>(Arc<Signal<T>>);

impl<T> SharedSignal<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(Signal::new(value)))
    }
}

impl<T: ?Sized> SharedSignal<T> {
    pub fn new_arc(value: Arc<T>) -> Self {
        Self(Arc::new(Signal::new_arc(value)))
    }
}

impl<T: ?Sized> Clone for SharedSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> Deref for SharedSignal<T> {
    type Target = Signal<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug + ?Sized> Debug for ReadSignal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ReadSignal").field(&self.get()).finish()
    }
}

impl<T: Debug + ?Sized> Debug for Signal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Signal").field(&self.get()).finish()
    }
}

impl<T: Debug + ?Sized> Debug for SharedSignal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SharedSignal").field(&self.get()).finish()
    }
}

impl<T: Default> Default for ReadSignal<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Default> Default for Signal<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Default> Default for SharedSignal<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: PartialEq + ?Sized> PartialEq for ReadSignal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl<T: PartialEq + ?Sized> PartialEq for Signal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl<T: PartialEq + ?Sized> PartialEq for SharedSignal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl<T: PartialEq + ?Sized> PartialEq<T> for ReadSignal<T> {
    fn eq(&self, other: &T) -> bool {
        self.get().as_ref() == other
    }
}

impl<T: PartialEq + ?Sized> PartialEq<T> for Signal<T> {
    fn eq(&self, other: &T) -> bool {
        self.get().as_ref() == other
    }
}

impl<T: PartialEq + ?Sized> PartialEq<T> for SharedSignal<T> {
    fn eq(&self, other: &T) -> bool {
        self.get().as_ref() == other
    }
}

impl<T: Eq + ?Sized> Eq for ReadSignal<T> {}
impl<T: Eq + ?Sized> Eq for Signal<T> {}
impl<T: Eq + ?Sized> Eq for SharedSignal<T> {}

impl<T: PartialOrd + ?Sized> PartialOrd for ReadSignal<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }
}

impl<T: PartialOrd + ?Sized> PartialOrd for Signal<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }
}

impl<T: PartialOrd + ?Sized> PartialOrd for SharedSignal<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }
}

impl<T: PartialOrd + ?Sized> PartialOrd<T> for ReadSignal<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.get().as_ref().partial_cmp(other)
    }
}

impl<T: PartialOrd + ?Sized> PartialOrd<T> for Signal<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.get().as_ref().partial_cmp(other)
    }
}

impl<T: PartialOrd + ?Sized> PartialOrd<T> for SharedSignal<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.get().as_ref().partial_cmp(other)
    }
}

impl<T: Ord + ?Sized> Ord for ReadSignal<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(&other.get())
    }
}

impl<T: Ord + ?Sized> Ord for Signal<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(&other.get())
    }
}

impl<T: Ord + ?Sized> Ord for SharedSignal<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(&other.get())
    }
}

impl<T: Hash + ?Sized> Hash for ReadSignal<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

impl<T: Hash + ?Sized> Hash for Signal<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

impl<T: Hash + ?Sized> Hash for SharedSignal<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}