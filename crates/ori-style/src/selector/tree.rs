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
    /// The cache keys for the style tree.
    pub keys: Vec<StyleCacheKey>,
}

impl StyleTree {
    /// Creates a new [`StyleTree`].
    #[inline(always)]
    pub fn new(element: Style) -> Self {
        let key = Self::hash_style(None, &element);

        Self {
            ancestors: Vec::new(),
            element,
            keys: vec![key],
        }
    }

    /// Creates a new [`StyleTree`] with a default root element.
    #[inline(always)]
    pub fn root() -> Self {
        let element = Style::default().with_tag("root");
        Self::new(element)
    }

    /// Pushes an ancestor to the tree.
    #[inline(always)]
    pub fn push(&mut self, element: Style) {
        let parent = self.keys.last().copied();
        let key = Self::hash_style(parent, &element);

        let parent = mem::replace(&mut self.element, element);
        self.ancestors.push(parent);
        self.keys.push(key);
    }

    /// Pops an ancestor from the tree.
    #[inline(always)]
    pub fn pop(&mut self) -> Option<Style> {
        self.keys.pop()?;
        let ancestor = self.ancestors.pop()?;
        Some(mem::replace(&mut self.element, ancestor))
    }

    /// Returns the parent tree.
    #[inline(always)]
    pub fn parent(&self) -> Option<StyleTree> {
        let mut tree = self.clone();
        tree.pop()?;
        Some(tree)
    }

    #[inline(always)]
    fn hash_style(parent: Option<StyleCacheKey>, style: &Style) -> StyleCacheKey {
        let mut hasher = seahash::SeaHasher::default();

        Hash::hash(&parent, &mut hasher);
        Hash::hash(&style.element, &mut hasher);
        Hash::hash(&style.classes, &mut hasher);
        Hash::hash(&style.tags, &mut hasher);

        StyleCacheKey::from_hash(hasher.finish())
    }

    /// Returns the cache key for the style tree.
    #[inline(always)]
    pub fn cache_key(&self) -> StyleCacheKey {
        self.keys.last().copied().unwrap_or(StyleCacheKey::root())
    }
}
