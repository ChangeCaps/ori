use std::sync::Arc;

use ori_reactive::OwnedSignal;

use crate::{AnyElement, Element, Node, NodeElement};

enum ViewKind<V: NodeElement> {
    Element(Node<V>),
    Fragment(Arc<[View<V>]>),
    Dynamic(OwnedSignal<View<V>>),
}

impl<V: NodeElement> Clone for ViewKind<V> {
    fn clone(&self) -> Self {
        match self {
            Self::Element(element) => Self::Element(element.clone()),
            Self::Fragment(fragment) => Self::Fragment(fragment.clone()),
            Self::Dynamic(signal) => Self::Dynamic(signal.clone()),
        }
    }
}

/// A trait for types that can be converted into a [`View`].
pub trait IntoView<V: NodeElement = Box<dyn AnyElement>> {
    /// Converts `self` into a [`Node`].
    fn into_view(self) -> View<V>;
}

impl<V: Element> IntoView<V> for V {
    fn into_view(self) -> View<V> {
        View::node(Node::new(self))
    }
}

impl<V: Element> IntoView for V {
    fn into_view(self) -> View {
        View::node(Node::new(Box::new(self)))
    }
}

impl<V: NodeElement> IntoView<V> for View<V> {
    fn into_view(self) -> View<V> {
        self
    }
}

impl<V: NodeElement> IntoView<V> for Node<V> {
    fn into_view(self) -> View<V> {
        View::node(self)
    }
}

/// A view in the UI tree.
///
/// A view can be one of the following:
/// - An [`Element`].
/// - A fragment containing a list of [`View`]s.
/// - A dynamic [`View`] that can change over time.
pub struct View<V: NodeElement = Box<dyn AnyElement>> {
    kind: ViewKind<V>,
}

impl<V: NodeElement> Clone for View<V> {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind.clone(),
        }
    }
}

impl<V: NodeElement> View<V> {
    fn from_kind(kind: ViewKind<V>) -> Self {
        Self { kind }
    }

    /// Creates a new [`Node`].
    pub fn new(into_node: impl IntoView<V>) -> Self {
        into_node.into_view()
    }

    /// Creates a new [`View`] from an [`Node`].
    pub fn node(element: Node<V>) -> Self {
        Self::from_kind(ViewKind::Element(element))
    }

    /// Creates a new [`View`] fragment from a list of [`View`]s.
    pub fn fragment(fragment: impl Into<Arc<[View<V>]>>) -> Self {
        Self::from_kind(ViewKind::Fragment(fragment.into()))
    }

    /// Creates a new dynamic [`View`] from an [`OwnedSignal`].
    pub fn dynamic(signal: OwnedSignal<View<V>>) -> Self {
        Self::from_kind(ViewKind::Dynamic(signal))
    }

    /// Creates an empty [`View`].
    pub fn empty() -> Self {
        Self::fragment(Vec::new())
    }

    /// Returns the number of elements in the [`View`].
    pub fn len(&self) -> usize {
        match &self.kind {
            ViewKind::Element(_) => 1,
            ViewKind::Fragment(fragment) => fragment.iter().map(View::len).sum(),
            ViewKind::Dynamic(signal) => signal.get().len(),
        }
    }

    /// Returns `true` if the [`Node`] contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// If `self` is a node, returns a reference to the [`Node`].
    pub fn get_node(&self) -> Option<&Node<V>> {
        match &self.kind {
            ViewKind::Element(element) => Some(element),
            _ => None,
        }
    }

    /// Tries to convert the [`View`] into a [`Node`].
    pub fn into_node(self) -> Option<Node<V>> {
        match self.kind {
            ViewKind::Element(element) => Some(element),
            _ => None,
        }
    }

    /// Tries to convert the [`View`] into a fragment.
    pub fn into_fragment(self) -> Option<Arc<[View<V>]>> {
        match self.kind {
            ViewKind::Fragment(fragment) => Some(fragment),
            _ => None,
        }
    }

    /// Tries to convert the [`View`] into a dynamic [`View`].
    pub fn into_dynamic(self) -> Option<OwnedSignal<View<V>>> {
        match self.kind {
            ViewKind::Dynamic(signal) => Some(signal),
            _ => None,
        }
    }

    /// Returns all elements in the [`View`], including nested elements, flattened into a single
    /// [`Vec`]. Dynamic [`View`]s are fetched in a reactive manner.
    pub fn flatten(&self) -> Vec<Node<V>> {
        match &self.kind {
            ViewKind::Element(element) => vec![element.clone()],
            ViewKind::Fragment(fragment) => fragment.iter().flat_map(View::flatten).collect(),
            ViewKind::Dynamic(signal) => signal.get().flatten(),
        }
    }

    /// Calls the given closure on all elements in the [`View`], including nested elements.
    /// Dynamic [`View`]s are fetched in a reactive manner.
    pub fn visit(&self, mut visitor: impl FnMut(&Node<V>)) {
        self.visit_inner(&mut visitor);
    }

    fn visit_inner(&self, visitor: &mut impl FnMut(&Node<V>)) {
        match &self.kind {
            ViewKind::Element(element) => visitor(element),
            ViewKind::Fragment(fragment) => {
                fragment.iter().for_each(|view| view.visit_inner(visitor))
            }
            ViewKind::Dynamic(signal) => signal.get().visit_inner(visitor),
        }
    }
}
