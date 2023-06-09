use std::sync::Arc;

use ori_reactive::OwnedSignal;

use crate::{Element, Node};

#[derive(Clone, Debug)]
enum ViewKind {
    Element(Node),
    Fragment(Arc<[View]>),
    Dynamic(OwnedSignal<View>),
}

impl<T: Element> From<T> for View {
    fn from(element: T) -> Self {
        Self::node(Node::new(element))
    }
}

impl Default for ViewKind {
    fn default() -> Self {
        Self::Fragment(Arc::new([]))
    }
}

/// A view in the UI tree.
///
/// A view can be one of the following:
/// - An [`Element`].
/// - A fragment containing a list of [`View`]s.
/// - A dynamic [`View`] that can change over time.
#[derive(Clone, Debug, Default)]
pub struct View {
    kind: ViewKind,
}

impl View {
    fn from_kind(kind: ViewKind) -> Self {
        Self { kind }
    }

    /// Creates a new [`Node`].
    pub fn new(into_node: impl Into<View>) -> Self {
        into_node.into()
    }

    /// Creates a new [`View`] from an [`Node`].
    pub fn node(element: Node) -> Self {
        Self::from_kind(ViewKind::Element(element))
    }

    /// Creates a new [`View`] fragment from a list of [`View`]s.
    pub fn fragment(fragment: impl Into<Arc<[View]>>) -> Self {
        Self::from_kind(ViewKind::Fragment(fragment.into()))
    }

    /// Creates a new dynamic [`View`] from an [`OwnedSignal`].
    pub fn dynamic(signal: OwnedSignal<View>) -> Self {
        Self::from_kind(ViewKind::Dynamic(signal))
    }

    /// Creates an empty fragment.
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
    pub fn get_node(&self) -> Option<&Node> {
        match &self.kind {
            ViewKind::Element(element) => Some(element),
            _ => None,
        }
    }

    /// Tries to convert the [`View`] into a [`Node`].
    pub fn into_node(self) -> Option<Node> {
        match self.kind {
            ViewKind::Element(element) => Some(element),
            _ => None,
        }
    }

    /// Tries to convert the [`View`] into a fragment.
    pub fn into_fragment(self) -> Option<Arc<[View]>> {
        match self.kind {
            ViewKind::Fragment(fragment) => Some(fragment),
            _ => None,
        }
    }

    /// Tries to convert the [`View`] into a dynamic [`View`].
    pub fn into_dynamic(self) -> Option<OwnedSignal<View>> {
        match self.kind {
            ViewKind::Dynamic(signal) => Some(signal),
            _ => None,
        }
    }

    /// Returns all elements in the [`View`], including nested elements, flattened into a single
    /// [`Vec`]. Dynamic [`View`]s are fetched in a reactive manner.
    pub fn flatten(&self) -> Vec<Node> {
        match &self.kind {
            ViewKind::Element(element) => vec![element.clone()],
            ViewKind::Fragment(fragment) => fragment.iter().flat_map(View::flatten).collect(),
            ViewKind::Dynamic(signal) => signal.get().flatten(),
        }
    }

    /// Calls the given `visitor` on all elements in the [`View`], including nested elements.
    /// Dynamic [`View`]s are fetched in a reactive manner.
    pub fn visit(&self, mut visitor: impl FnMut(&Node)) {
        self.visit_inner(&mut visitor);
    }

    fn visit_inner(&self, visitor: &mut impl FnMut(&Node)) {
        match &self.kind {
            ViewKind::Element(element) => visitor(element),
            ViewKind::Fragment(fragment) => {
                fragment.iter().for_each(|view| view.visit_inner(visitor))
            }
            ViewKind::Dynamic(signal) => signal.get().visit_inner(visitor),
        }
    }
}

impl<T: Into<View>> From<Vec<T>> for View {
    fn from(views: Vec<T>) -> Self {
        Self::fragment(views.into_iter().map(Into::into).collect::<Vec<_>>())
    }
}

impl<T: Clone + Into<View>> From<&[T]> for View {
    fn from(views: &[T]) -> Self {
        Self::fragment(views.iter().cloned().map(Into::into).collect::<Vec<_>>())
    }
}

impl From<Arc<[View]>> for View {
    fn from(views: Arc<[View]>) -> Self {
        Self::fragment(views)
    }
}

impl From<OwnedSignal<View>> for View {
    fn from(dynamic: OwnedSignal<View>) -> Self {
        Self::from_kind(ViewKind::Dynamic(dynamic))
    }
}

impl<T: Into<View>> From<Option<T>> for View {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Self::empty(),
        }
    }
}
