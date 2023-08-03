use ori_reactive::{Scope, Signal};

use crate::{Element, Node};

/// A reference to a [`Node`].
#[derive(Clone, Copy, Debug)]
pub struct NodeRef {
    node: Signal<Option<Node>>,
}

impl NodeRef {
    /// Creates a new [`NodeRef`].
    pub fn new(cx: Scope) -> Self {
        Self {
            node: cx.signal(None),
        }
    }

    /// Tries to get the [`Node`].
    pub fn try_get(&self) -> Option<Node> {
        self.node.try_get().flatten()
    }

    /// Gets the [`Node`].
    ///
    /// # Panics
    /// - If the [`NodeRef`] is empty.
    #[track_caller]
    pub fn get(&self) -> Node {
        self.node.get().expect("NodeRef is empty")
    }

    /// Tries to get the [`Node`] as a specific [`Element`].
    pub fn downcast<T: Element, U>(&self, f: impl FnOnce(&mut T) -> U) -> Option<U> {
        self.try_get()?.downcast(f).ok()
    }

    /// Sets the [`Node`].
    pub fn set(&self, node: Node) {
        self.node.set(Some(node));
    }

    /// Tracks the [`Node`].
    pub fn track(&self) {
        self.node.track();
    }

    /// Clears the [`Node`].
    pub fn clear(&self) {
        self.node.set(None);
    }
}
