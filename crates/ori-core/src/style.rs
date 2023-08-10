use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{Theme, Value};

pub struct Key<T> {
    name: &'static str,
    marker: PhantomData<fn(&T)>,
}

impl<T> Key<T> {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            marker: PhantomData,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn cast<U>(self) -> Key<U> {
        Key {
            name: self.name,
            marker: PhantomData,
        }
    }
}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            marker: PhantomData,
        }
    }
}

impl<T> Copy for Key<T> {}

impl<T> Debug for Key<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key").field("name", &self.name).finish()
    }
}

impl<T> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T> Eq for Key<T> {}

impl<T> Hash for Key<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Style<T> {
    Concrete(T),
    Key(Key<T>),
}

impl<T: Default> Default for Style<T> {
    fn default() -> Self {
        Self::Concrete(T::default())
    }
}

impl<T> From<T> for Style<T> {
    fn from(value: T) -> Self {
        Self::Concrete(value)
    }
}

impl<T> From<T> for Style<Option<T>> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> From<Key<T>> for Style<T> {
    fn from(key: Key<T>) -> Self {
        Self::Key(key)
    }
}

impl<T> From<Key<T>> for Option<Style<T>> {
    fn from(value: Key<T>) -> Self {
        Some(Style::new(value))
    }
}

impl<T> Style<T> {
    pub fn new(value: impl Into<Self>) -> Self {
        value.into()
    }

    pub fn get(&self, theme: &Theme) -> T
    where
        T: Clone + Default + Value,
    {
        theme.style(self)
    }

    pub const fn key(name: &'static str) -> Self {
        Self::Key(Key::new(name))
    }
}

pub trait Styled<T> {
    fn style(self) -> Style<T>;
}

impl<T: Into<U>, U> Styled<U> for T {
    fn style(self) -> Style<U> {
        Style::new(self.into())
    }
}
