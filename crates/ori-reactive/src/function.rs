use std::{any::Any, future::Future};

use crate::{effect, Emitter, Event, EventSink, OwnedSignal, ReadSignal, Scope, Signal};

/// Runs a callback without tracking any signals.
pub fn untrack<T>(f: impl FnOnce() -> T) -> T {
    effect::untrack(f)
}

/// Gets the [`EventSink`] for a [`Scope`].
pub fn event_sink(cx: Scope) -> EventSink {
    cx.event_sink()
}

/// Gets the [`Emitter`] for a [`Scope`].
pub fn event_emitter(cx: Scope) -> Emitter<Event> {
    cx.event_emitter()
}

/// Emits an event.
pub fn emit(cx: Scope, event: impl Any + Send + Sync) {
    cx.emit(event)
}

/// Registers a callback for an event.
pub fn on<T: Any + Send + Sync>(cx: Scope, callback: impl FnMut(&T) + Send + 'static) {
    cx.on::<T>(callback)
}

/// Pushes a context to the [`Scope`].
pub fn push_context(cx: Scope, context: impl Any + Send + Sync) {
    cx.with_context(context);
}

/// Checks is a context is present in the [`Scope`].
pub fn has_context<C: Any + Send + Sync>(cx: Scope) -> bool {
    cx.has_context::<C>()
}

/// Tries to get a context from the [`Scope`].
pub fn get_context<C: Any + Clone + Send + Sync>(cx: Scope) -> Option<C> {
    cx.get_context::<C>()
}

/// Gets a context from the [`Scope`].
///
/// # Panics
/// - If the context is not present.
#[track_caller]
pub fn context<C: Any + Clone + Send + Sync>(cx: Scope) -> C {
    cx.context::<C>()
}

/// Spawns a future on the [`Scope`].
pub fn spawn_future(cx: Scope, task: impl Future<Output = ()> + Send + 'static) {
    cx.spawn(task)
}

/// Creates a new [`Signal`] with the given value on the [`Scope`].
pub fn signal<T: Send + Sync + 'static>(cx: Scope, value: T) -> Signal<T> {
    cx.signal(value)
}

/// Creates an effect that runs the given callback on the [`Scope`].
///
/// Effects are rerun when any of the signals they depend on change.
#[track_caller]
pub fn effect(cx: Scope, f: impl FnMut() + Send + 'static) {
    cx.effect(f)
}

/// Creates an effect that takes a child [`Scope`] and runs the given callback on it.
///
/// See [`effect`](effect()) for more information.
#[track_caller]
pub fn effect_scoped(cx: Scope, f: impl FnMut(Scope) + Send + 'static) {
    cx.effect_scoped(f)
}

/// Creates a memorized [`ReadSignal`], the value of which is recomputed when any of the signals it
/// depends on change.
#[track_caller]
pub fn memo<T: Send + Sync>(cx: Scope, f: impl FnMut() -> T + Send + 'static) -> ReadSignal<T> {
    cx.memo(f)
}

/// Creates a memorized [`OwnedSignal`], see [`memo`] for more information.
#[track_caller]
pub fn owned_memo<T: Send + Sync>(
    cx: Scope,
    f: impl FnMut() -> T + Send + 'static,
) -> OwnedSignal<T> {
    cx.owned_memo(f)
}

/// Creates a memorized [`ReadSignal`] that takes a child [`Scope`], see [`memo`] for more
/// information.
#[track_caller]
pub fn memo_scoped<T: Send + Sync>(
    cx: Scope,
    f: impl FnMut(Scope) -> T + Send + 'static,
) -> ReadSignal<T> {
    cx.memo_scoped(f)
}

/// Creates a memorized [`OwnedSignal`] that takes a child [`Scope`], see [`memo`] for more
/// information.
#[track_caller]
pub fn owned_memo_scoped<T: Send + Sync>(
    cx: Scope,
    f: impl FnMut(Scope) -> T + Send + 'static,
) -> OwnedSignal<T> {
    cx.owned_memo_scoped(f)
}
