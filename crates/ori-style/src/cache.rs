use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash, Hasher},
    mem::MaybeUninit,
};

use crate::{StyleRuleAttribute, StyleSpec};

/// A hash of a [`StyleTree`](crate::StyleTree).
///
/// This is used as a key in a [`StyleCache`].
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StyleCacheKey {
    hash: u64,
}

impl StyleCacheKey {
    /// Creates a new [`StyleCacheKey`] from a hash.
    pub const fn from_hash(hash: u64) -> Self {
        Self { hash }
    }
}

/// A [`Hasher`] for [`StyleCacheKey`]. This will directly write the hash into
/// the hasher, instead of hashing the bytes of the key.
///
/// This struct is inherently unsafe, an *must* therefore never be exposed outside
/// of this module. When dealing with this struct, be careful, and always make sure
/// to follow the safety requirements below.
///
/// # Safety
/// - *Must* only be used to hash a [`StyleCacheKey`], an *no* other type.
/// - Calling [`Hasher::finish`] before writing a hash will result in undefined behavior.
#[repr(transparent)]
#[derive(Clone, Copy)]
struct StyleCacheHasher {
    hash: MaybeUninit<u64>,
}

impl Default for StyleCacheHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl StyleCacheHasher {
    pub const fn new() -> Self {
        Self {
            hash: MaybeUninit::uninit(),
        }
    }
}

impl Hasher for StyleCacheHasher {
    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!()
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        self.hash = MaybeUninit::new(i);
    }

    #[inline(always)]
    fn finish(&self) -> u64 {
        unsafe { self.hash.assume_init() }
    }
}

impl BuildHasher for StyleCacheHasher {
    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        Self::new()
    }
}

/// An entry in the style cache.
#[derive(Clone, Debug)]
pub struct StyleCacheEntry {
    /// The location of the attribute.
    pub attribute: StyleRuleAttribute,
    /// The specifity of the attribute.
    pub specificity: StyleSpec,
}

/// A cache of style attributes.
#[derive(Debug, Default)]
pub struct StyleCache {
    attributes: HashMap<u64, Option<StyleCacheEntry>, StyleCacheHasher>,
}

impl Clone for StyleCache {
    fn clone(&self) -> Self {
        Self {
            attributes: self.attributes.clone(),
        }
    }
}

impl StyleCache {
    /// Create a new style cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear the style cache.
    pub fn clear(&mut self) {
        #[cfg(feature = "tracing")]
        tracing::trace!("Clearing style cache");
        self.attributes.clear();
    }

    fn hash(hash: StyleCacheKey, key: &str) -> u64 {
        let mut hasher = seahash::SeaHasher::default();
        Hash::hash(key, &mut hasher);

        hasher.finish() ^ hash.hash
    }

    /// Insert a style attribute into the cache.
    pub fn insert(&mut self, cache_key: StyleCacheKey, key: &str, entry: Option<StyleCacheEntry>) {
        let hash = Self::hash(cache_key, key);

        #[cfg(all(debug_assertions, feature = "tracing"))]
        if self.attributes.contains_key(&hash) {
            tracing::warn!(
                "Overwriting style cache entry for {}, this might be a hash collision",
                key
            );
        }

        self.attributes.insert(hash, entry);
    }

    /// Get a style attribute from the cache.
    pub fn get(&self, hash: StyleCacheKey, key: &str) -> Option<Option<StyleCacheEntry>> {
        let hash = Self::hash(hash, key);
        self.attributes.get(&hash).cloned()
    }
}
