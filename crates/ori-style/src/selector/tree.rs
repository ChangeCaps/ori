use std::{
    hash::{Hash, Hasher},
    mem,
};

use crate::{Style, StyleCacheKey};

/// A style selector tree.
///
/// This is used to match a style selector against an element.
#[derive(Clone, Debug)]
pub struct StyleTree {
    /// The ancestors of the selector.
    pub ancestors: Vec<Style>,
    /// The top level element selector.
    pub element: Style,
}

impl StyleTree {
    /// Creates a new [`StyleTree`].
    pub fn new(element: Style) -> Self {
        Self {
            ancestors: Vec::new(),
            element,
        }
    }

    /// Pushes an ancestor to the tree.
    pub fn push(&mut self, element: Style) {
        let ancestor = mem::replace(&mut self.element, element);
        self.ancestors.push(ancestor);
    }

    /// Pops an ancestor from the tree.
    pub fn pop(&mut self) -> Option<Style> {
        let ancestor = self.ancestors.pop()?;
        Some(mem::replace(&mut self.element, ancestor))
    }

    /// Returns the parent tree.
    pub fn parent(&self) -> Option<StyleTree> {
        let mut tree = self.clone();
        tree.pop()?;
        Some(tree)
    }

    fn hash_style(style: &Style, hasher: &mut seahash::SeaHasher) {
        Hash::hash(&style.element, hasher);
        Hash::hash(&style.classes, hasher);
        Hash::hash(&style.tags, hasher);
    }

    /// Returns the cache key for the style tree.
    pub fn cache_key(&self) -> StyleCacheKey {
        let mut hasher = seahash::SeaHasher::default();

        for ancestor in &self.ancestors {
            Self::hash_style(ancestor, &mut hasher);
        }

        Self::hash_style(&self.element, &mut hasher);

        StyleCacheKey::from_hash(hasher.finish())
    }
}
