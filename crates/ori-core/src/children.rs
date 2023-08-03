use std::slice;

use deref_derive::{Deref, DerefMut};
use glam::Vec2;
use ori_graphics::Rect;
use ori_reactive::Event;
use smallvec::{smallvec, SmallVec};

use crate::{
    AvailableSpace, DrawContext, EventContext, FlexLayout, IntoView, LayoutContext, Node, Parent,
    View,
};

/// Children of an [`Element`](crate::Element).
#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct Children {
    elements: SmallVec<[View; 1]>,
}

impl<T: IntoView> From<T> for Children {
    fn from(value: T) -> Self {
        Self {
            elements: smallvec![value.into_view()],
        }
    }
}

impl Parent for Children {
    fn clear_children(&mut self) {
        self.elements.clear();
    }

    fn add_children(&mut self, children: impl Iterator<Item = View>) -> usize {
        let children = children.collect::<Vec<_>>();
        self.elements.push(View::fragment(children));
        self.elements.len() - 1
    }

    fn set_children(&mut self, slot: usize, children: impl Iterator<Item = View>) {
        let children = children.collect::<Vec<_>>();
        self.elements[slot] = View::fragment(children);
    }
}

impl Children {
    /// Create a new children.
    pub const fn new() -> Self {
        Self {
            elements: SmallVec::new_const(),
        }
    }

    /// Returns the amount of children.
    pub fn len(&self) -> usize {
        self.iter().map(View::len).sum()
    }

    /// Returns `true` if there are no children.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Extend the children with an iterator.
    pub fn extend(&mut self, children: impl IntoIterator<Item = View>) {
        self.elements.extend(children);
    }

    /// Layout the children using a FlexLayout.
    pub fn flex_layout(
        &self,
        cx: &mut LayoutContext,
        space: AvailableSpace,
        flex: FlexLayout,
    ) -> Vec2 {
        let padded_space = space.shrink(flex.padding.size());

        cx.with_space(padded_space, |cx| {
            self.flex_layout_padded(cx, padded_space, flex) + flex.padding.size()
        })
    }

    /// Returns true if any child needs to be laid out.
    pub fn needs_layout(&self) -> bool {
        self.nodes().any(|child| child.needs_layout())
    }

    /// Returns the local rect of the flex container.
    pub fn local_rect(&self) -> Rect {
        let mut rect = None;

        for child in self.nodes() {
            let rect = rect.get_or_insert_with(|| child.local_rect());
            *rect = rect.union(child.local_rect());
        }

        rect.unwrap_or_default()
    }

    /// Returns the global rect of the flex container.
    pub fn rect(&self) -> Rect {
        let mut rect = None;

        for child in self.nodes() {
            let rect = rect.get_or_insert_with(|| child.rect());
            *rect = rect.union(child.rect());
        }

        rect.unwrap_or_default()
    }

    /// Returns the size of the flex container.
    pub fn size(&self) -> Vec2 {
        self.rect().size()
    }

    /// Returns the offset of the flex container.
    pub fn set_offset(&self, offset: Vec2) {
        if self.is_empty() {
            return;
        }

        let min = self.local_rect().min;

        for child in self.nodes() {
            let child_offset = child.local_rect().min - min;
            child.set_offset(child_offset + offset);
        }
    }

    /// Call the `event` method on all the children.
    pub fn event(&self, cx: &mut EventContext, event: &Event) {
        for child in self.nodes() {
            child.event(cx, event);
        }
    }

    /// Draws the flex container.
    pub fn draw(&self, cx: &mut DrawContext) {
        for child in self.nodes() {
            child.draw(cx);
        }
    }

    /// Returns an iterator over all the children in the flex container.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &View> {
        self.into_iter()
    }

    /// Returns an iterator over all the nodes in the flex container.
    pub fn nodes(&self) -> impl DoubleEndedIterator<Item = Node> + '_ {
        self.iter().flat_map(|child| child.flatten())
    }
}

impl IntoIterator for Children {
    type Item = View;
    type IntoIter = smallvec::IntoIter<[Self::Item; 1]>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a> IntoIterator for &'a Children {
    type Item = &'a View;
    type IntoIter = slice::Iter<'a, View>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}
