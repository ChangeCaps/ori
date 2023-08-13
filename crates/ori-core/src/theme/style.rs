use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    marker::PhantomData,
    sync::Arc,
};

use crate::{Theme, ThemeError, Value};

/// A key used to store and retrieve values from a [`Theme`].
pub struct Key<T> {
    name: &'static str,
    marker: PhantomData<fn(&T)>,
}

impl<T> Key<T> {
    /// Creates a new key with the given name.
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            marker: PhantomData,
        }
    }

    /// Returns the name of the key.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Casts the key to a different type.
    ///
    /// This is almost never useful.
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

/// A trait for any type that can retrieve a value from a [`Theme`].
///
/// This is implemented for `Fn(&Theme) -> Result<T, ThemeError> + Send + Sync + 'static` by
/// default.
pub trait AnyStyle<T>: Send + Sync + 'static {
    fn get(&self, theme: &Theme) -> Result<T, ThemeError>;
}

impl<T: Value, F: Fn(&Theme) -> Result<T, ThemeError> + Send + Sync + 'static> AnyStyle<T> for F {
    fn get(&self, theme: &Theme) -> Result<T, ThemeError> {
        self(theme)
    }
}

/// A style can be either a concrete value, a key, or a function that returns a value retrieved from
/// a theme.
#[derive(Clone)]
pub enum Style<T> {
    /// A concrete value.
    Concrete(T),
    /// A key that gets a value from a theme.
    Key(Key<T>),
    /// A function that retrieves a value from a theme.
    Any(Arc<dyn AnyStyle<T>>),
}

impl<T: Debug> Debug for Style<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Style::Concrete(value) => f.debug_tuple("Concrete").field(value).finish(),
            Style::Key(key) => f.debug_tuple("Key").field(key).finish(),
            Style::Any(_) => f
                .debug_tuple("Any")
                .field(&"fn(&Theme) -> Result<T, ThemeError>")
                .finish(),
        }
    }
}

impl<T: PartialEq> PartialEq for Style<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Concrete(a), Self::Concrete(b)) => a == b,
            (Self::Key(a), Self::Key(b)) => a == b,
            _ => false,
        }
    }
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
    /// Create a new style from the given value.
    pub fn new(styled: impl Styled<T>) -> Self {
        styled.style()
    }

    /// Create a new style from the given key name.
    pub const fn key(name: &'static str) -> Self {
        Self::Key(Key::new(name))
    }

    /// Get the value from a theme.
    pub fn get(&self, theme: &Theme) -> T
    where
        T: Clone + Default + Value,
    {
        theme.style(self)
    }
}

/// A trait for any type that can be converted into a [`Style`].
pub trait Styled<T> {
    /// Convert the type into a [`Style`].
    fn style(self) -> Style<T>;
}

impl<T: Into<Style<U>>, U> Styled<U> for T {
    fn style(self) -> Style<U> {
        self.into()
    }
}
