use std::{borrow::Cow, slice};

use deref_derive::{Deref, DerefMut};
use glam::Vec2;
use ori_graphics::Rect;
use ori_reactive::Event;
use smallvec::{smallvec, SmallVec};

use crate::{
    AvailableSpace, DrawContext, EventContext, FlexLayout, IntoView, LayoutContext, Node, Parent,
    View,
};

const CHILDREN_CAPACITY: usize = 2;

/// Children of an [`Element`](crate::Element).
#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct Children {
    views: SmallVec<[View; CHILDREN_CAPACITY]>,
}

impl<T: IntoView> From<T> for Children {
    fn from(value: T) -> Self {
        Self {
            views: smallvec![value.into_view()],
        }
    }
}

impl Parent for Children {
    fn clear_children(&mut self) {
        self.views.clear();
    }

    fn add_children(&mut self, children: impl Iterator<Item = View>) -> usize {
        let children = children.collect::<Vec<_>>();
        self.views.push(View::fragment(children));
        self.views.len() - 1
    }

    fn set_children(&mut self, slot: usize, children: impl Iterator<Item = View>) {
        let children = children.collect::<Vec<_>>();
        self.views[slot] = View::fragment(children);
    }
}

impl Children {
    /// Create a new children.
    pub const fn new() -> Self {
        Self {
            views: SmallVec::new_const(),
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
        self.views.extend(children);
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
        let mut needs_layout = false;

        self.visit(|child| {
            needs_layout |= child.needs_layout();
        });

        needs_layout
    }

    /// Returns the local rect of the flex container.
    pub fn local_rect(&self) -> Rect {
        let mut rect = None;

        self.visit(|child| {
            let rect = rect.get_or_insert_with(|| child.local_rect());
            *rect = rect.union(child.local_rect());
        });

        rect.unwrap_or_default()
    }

    /// Returns the global rect of the flex container.
    pub fn rect(&self) -> Rect {
        let mut rect = None;

        self.visit(|child| {
            let rect = rect.get_or_insert_with(|| child.rect());
            *rect = rect.union(child.rect());
        });

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

        self.visit(|child| {
            let child_offset = child.local_rect().min - min;
            child.set_offset(child_offset + offset);
        });
    }

    /// Call the `event` method on all the children.
    pub fn event(&self, cx: &mut EventContext, event: &Event) {
        self.visit(|node| {
            node.event(cx, event);
        });
    }

    /// Draws the flex container.
    pub fn draw(&self, cx: &mut DrawContext) {
        self.visit(|node| {
            node.draw(cx);
        });
    }

    #[inline(always)]
    pub fn visit(&self, mut f: impl FnMut(&Node)) {
        for view in self.views.iter() {
            view.visit(&mut f);
        }
    }

    /// Returns an iterator over all the children in the flex container.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &View> {
        self.into_iter()
    }

    /// Returns an iterator over all the nodes in the flex container.
    pub fn nodes(&self) -> impl DoubleEndedIterator<Item = Cow<'_, Node>> + '_ {
        self.iter().flat_map(|child| child.flatten())
    }
}

impl IntoIterator for Children {
    type Item = View;
    type IntoIter = smallvec::IntoIter<[Self::Item; CHILDREN_CAPACITY]>;

    fn into_iter(self) -> Self::IntoIter {
        self.views.into_iter()
    }
}

impl<'a> IntoIterator for &'a Children {
    type Item = &'a View;
    type IntoIter = slice::Iter<'a, View>;

    fn into_iter(self) -> Self::IntoIter {
        self.views.iter()
    }
}
