use ori_reactive::{Callback, Emitter, OwnedSignal, Scope, Signal};

use crate::View;

/// A trait for types that can be built from a [`View`].
pub trait Build {
    /// The properties type.
    type Properties<'a>;

    /// The events type.
    type Events<'a>;

    /// The bindings type.
    type Bindings<'a>;

    /// Builds the default view.
    fn build() -> View;

    /// Retrieves the properties of the view.
    fn properties(view: &View, f: impl FnOnce(Self::Properties<'_>));

    /// Retrieves the events of the view.
    fn events(view: &View, f: impl FnOnce(Self::Events<'_>));

    /// Retrieves the bindings of the view.
    fn bindings(view: &View, f: impl FnOnce(Self::Bindings<'_>));
}

/// A trait that is implemented for every type a callback can be subscribed to.
pub trait BindCallback {
    /// The event type.
    type Event;

    /// Binds a callback to the signal.
    fn bind(&mut self, cx: Scope, callback: impl FnMut(&Self::Event) + Send + 'static);
}

impl<T> BindCallback for Emitter<T> {
    type Event = T;

    fn bind(&mut self, cx: Scope, callback: impl FnMut(&Self::Event) + Send + 'static) {
        let callback = Callback::new(callback);
        self.subscribe(&callback);
        cx.manage_callback(callback);
    }
}

/// A trait implemented for every type that can be bound to a signal.
pub trait Bindable<'a> {
    /// The item type.
    type Item: Send;

    /// Binds the signal to the value.
    fn bind(&mut self, cx: Scope, signal: Signal<Self::Item>);
}

impl<'a, T: Send + Sync + Clone + 'static> Bindable<'a> for OwnedSignal<T> {
    type Item = T;

    fn bind(&mut self, _cx: Scope, signal: Signal<Self::Item>) {
        self.bind(signal);
    }
}

/// A trait for setting children on an element.
pub trait Parent {
    /// Clears all children.
    fn clear_children(&mut self);

    /// Adds `children` to a new slot and returns the slot index.
    fn add_children(&mut self, children: impl Iterator<Item = View>) -> usize;

    /// Sets the children of `slot` to `children`.
    fn set_children(&mut self, slot: usize, children: impl Iterator<Item = View>);

    /// Adds `child` to a new slot and returns the slot index.
    fn add_child(&mut self, child: impl Into<View>) -> usize {
        self.add_children(std::iter::once(View::new(child)))
    }

    /// Sets the children of `slot` to `child`.
    fn set_child(&mut self, slot: usize, child: impl Into<View>) {
        self.set_children(slot, std::iter::once(View::new(child)))
    }

    /// Adds `children` to a new slot.
    fn with_children(mut self, children: impl Iterator<Item = View>) -> Self
    where
        Self: Sized,
    {
        self.add_children(children);
        self
    }

    /// Adds `child` to a new slot.
    fn with_child(mut self, child: impl Into<View>) -> Self
    where
        Self: Sized,
    {
        self.add_child(child);
        self
    }
}
