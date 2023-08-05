use std::{borrow::Cow, sync::Arc};

use ori_reactive::OwnedSignal;
use smallvec::SmallVec;

use crate::{Element, Node};

const VIEW_FLATTEN_CAPACITY: usize = 8;

#[derive(Clone, Debug)]
enum ViewKind {
    Node(Node),
    Fragment(Arc<[View]>),
    Dynamic(OwnedSignal<View>),
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
    pub fn new(into_view: impl IntoView) -> Self {
        into_view.into_view()
    }

    /// Creates a new [`View`] from an [`Node`].
    pub fn node(node: Node) -> Self {
        Self::from_kind(ViewKind::Node(node))
    }

    /// Creates a new [`View`] fragment from a list of [`View`]s.
    pub fn fragment(fragment: impl IntoIterator<Item = View>) -> Self {
        Self::from_kind(ViewKind::Fragment(fragment.into_iter().collect()))
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
            ViewKind::Node(_) => 1,
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
            ViewKind::Node(element) => Some(element),
            _ => None,
        }
    }

    /// Tries to convert the [`View`] into a [`Node`].
    pub fn into_node(self) -> Option<Node> {
        match self.kind {
            ViewKind::Node(element) => Some(element),
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
    #[inline(always)]
    pub fn flatten(&self) -> SmallVec<[Cow<'_, Node>; VIEW_FLATTEN_CAPACITY]> {
        let mut buffer = SmallVec::new();
        self.flatten_inner(&mut buffer);
        buffer
    }

    #[inline(always)]
    fn flatten_inner<'a>(&'a self, buffer: &mut SmallVec<[Cow<'a, Node>; VIEW_FLATTEN_CAPACITY]>) {
        match &self.kind {
            ViewKind::Node(element) => buffer.push(Cow::Borrowed(element)),
            ViewKind::Fragment(fragment) => {
                for view in fragment.iter() {
                    view.flatten_inner(buffer);
                }
            }
            ViewKind::Dynamic(signal) => {
                signal.get().visit(|node| {
                    buffer.push(Cow::Owned(node.clone()));
                });
            }
        }
    }

    /// Calls the given `visitor` on all elements in the [`View`], including nested elements.
    /// Dynamic [`View`]s are fetched in a reactive manner.
    #[inline(always)]
    pub fn visit(&self, mut visitor: impl FnMut(&Node)) {
        self.visit_recurse(&mut visitor);
    }

    #[inline(always)]
    fn visit_recurse(&self, visitor: &mut impl FnMut(&Node)) {
        match &self.kind {
            ViewKind::Node(element) => visitor(element),
            ViewKind::Fragment(fragment) => {
                fragment.iter().for_each(|view| view.visit_recurse(visitor))
            }
            ViewKind::Dynamic(signal) => signal.get().visit_recurse(visitor),
        }
    }
}

/// A trait for types that can be converted into a [`View`].
///
/// This trait is implemented for all types that implement [`Into<View>`],
/// and should therefore not be implemented manually.
pub trait IntoView {
    fn into_view(self) -> View;
}

impl IntoView for View {
    fn into_view(self) -> View {
        self
    }
}

impl<T: Element> IntoView for T {
    fn into_view(self) -> View {
        View::node(Node::new(self))
    }
}

impl IntoView for Node {
    fn into_view(self) -> View {
        View::node(self)
    }
}

impl<T: IntoView> IntoView for Vec<T> {
    fn into_view(self) -> View {
        View::fragment(self.into_iter().map(IntoView::into_view))
    }
}

impl<T: Clone + IntoView> IntoView for &[T] {
    fn into_view(self) -> View {
        View::fragment(self.iter().cloned().map(IntoView::into_view))
    }
}

impl<T: IntoView, const LEN: usize> IntoView for [T; LEN] {
    fn into_view(self) -> View {
        View::fragment(self.into_iter().map(IntoView::into_view))
    }
}

macro_rules! impl_from_tuple {
    (@internal $($name:ident),*) => {
        impl<$($name: IntoView),*> IntoView for ($($name,)*) {
            fn into_view(self) -> View {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                View::fragment(vec![$($name.into_view(),)*])
            }
        }
    };
    ($first:ident $(, $name:ident)*) => {
        impl_from_tuple!(@internal $first $(,$name)*);
        impl_from_tuple!($($name),*);
    };
    () => {
        impl_from_tuple!(@internal);
    };
}

impl_from_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

impl IntoView for OwnedSignal<View> {
    fn into_view(self) -> View {
        View::dynamic(self)
    }
}

impl<T: IntoView> IntoView for Option<T> {
    fn into_view(self) -> View {
        match self {
            Some(view) => view.into_view(),
            None => View::empty(),
        }
    }
}
