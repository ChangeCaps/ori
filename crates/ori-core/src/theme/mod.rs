mod builtin_theme;
mod error;
mod palette;
mod style;

pub use error::*;
pub use palette::*;
pub use style::*;

use std::{any::Any, collections::HashMap, fmt::Debug, hash::BuildHasher, sync::Arc};

pub trait Value: Any + Send + Sync {}
impl<T: Any + Send + Sync> Value for T {}

#[derive(Clone, Copy, Default)]
struct ThemeHasher;

impl BuildHasher for ThemeHasher {
    type Hasher = seahash::SeaHasher;

    fn build_hasher(&self) -> Self::Hasher {
        seahash::SeaHasher::new()
    }
}

type ArcStr = Arc<str>;
type ArcValue = Arc<dyn Any + Send + Sync>;

/// A collection of [`Style`]s.
///
/// Each style is stored under a [`Key`] and can be retrieved using [`Theme::get`] or
/// [`Style::get`].
#[derive(Clone, Debug, Default)]
pub struct Theme {
    values: HashMap<ArcStr, Style<ArcValue>, ThemeHasher>,
}

impl Theme {
    const MAX_DEPTH: usize = 32;

    /// Creates a new empty theme.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new theme with the default values.
    pub fn builtin() -> Self {
        builtin_theme::builtin_theme()
    }

    /// Returns the number of values in the theme.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` if the theme contains no values.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Set a value wrapped in an [`Arc`].
    pub fn set_arc<T: Value>(&mut self, key: Key<T>, value: Arc<T>) {
        let name = ArcStr::from(key.name());
        let value: Style<ArcValue> = Style::Concrete(value);

        self.values.insert(name, value);
    }

    /// Set a key to another key.
    pub fn set_key<T: Value>(&mut self, key: Key<T>, value: Key<T>) {
        let name = ArcStr::from(key.name());
        let value: Style<ArcValue> = Style::Key(value.cast());

        self.values.insert(name, value);
    }

    /// Set a value wrapped in an [`AnyStyle`].
    pub fn set_any<T: Value>(&mut self, key: Key<T>, value: Arc<dyn AnyStyle<T>>) {
        let name = ArcStr::from(key.name());
        let value: Style<ArcValue> = Style::Any(Arc::new(move |theme: &Theme| {
            Ok(Arc::new(value.get(theme)?) as ArcValue)
        }));

        self.values.insert(name, value);
    }

    /// Set a value.
    pub fn set<T: Value>(&mut self, key: Key<T>, value: impl Styled<T>) {
        match value.style() {
            Style::Concrete(value) => {
                self.set_arc(key, Arc::new(value));
            }
            Style::Key(value) => {
                self.set_key(key, value);
            }
            Style::Any(value) => {
                self.set_any(key, value);
            }
        }
    }

    /// Map the value of one key to another.
    pub fn map<T: Value, U: Value>(
        &mut self,
        key: Key<U>,
        source: Key<T>,
        map: impl Fn(&T) -> U + Send + Sync + 'static,
    ) {
        let name = ArcStr::from(key.name());
        let map = Arc::new(move |theme: &Theme| {
            let value = theme.try_get_value(source.name())?;
            let value = value.downcast_ref::<T>().ok_or(ThemeError::Downcast)?;
            Ok(Arc::new(map(value)) as ArcValue)
        });

        self.values.insert(name, Style::Any(map));
    }

    /// Extend the theme with another theme.
    pub fn extend(&mut self, other: Self) {
        self.values.extend(other.values);
    }

    /// Clear the theme.
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Try to get a value.
    ///
    /// This is almost never useful and should be avoided.
    pub fn try_get_value(&self, mut key: &str) -> Result<ArcValue, ThemeError> {
        let mut depth = 0;

        while let Some(value) = self.values.get(key) {
            if depth >= Self::MAX_DEPTH {
                return Err(ThemeError::MaxDepth);
            }

            depth += 1;

            match value {
                Style::Concrete(value) => return Ok(value.clone()),
                Style::Key(other) => key = other.name(),
                Style::Any(any) => return any.get(self),
            }
        }

        Err(ThemeError::NotFound(ArcStr::from(key)))
    }

    /// Try to get a value.
    ///
    /// # Errors
    /// - [`ThemeError::NotFound`] if the key is not found.
    /// - [`ThemeError::Downcast`] if the value cannot be downcast to the requested type.
    /// - [`ThemeError::MaxDepth`] if the maximum depth is reached.
    pub fn try_get<T: Clone + Value>(&self, key: Key<T>) -> Result<T, ThemeError> {
        let value = self.try_get_value(key.name())?;
        (value.downcast_ref::<T>().cloned()).ok_or(ThemeError::Downcast)
    }

    /// Get a value.
    ///
    /// If an error occurs, the default value is returned, and a warning is logged.
    pub fn get<T: Clone + Default + Value>(&self, key: Key<T>) -> T {
        match self.try_get(key) {
            Ok(value) => value,
            Err(err) => {
                tracing::warn!("{}", err);
                T::default()
            }
        }
    }

    /// Try to resolve the value of a [`Style`].
    pub fn try_style<T: Clone + Value>(&self, style: &Style<T>) -> Result<T, ThemeError> {
        match style {
            Style::Concrete(value) => Ok(value.clone()),
            Style::Key(key) => self.try_get(*key),
            Style::Any(any) => any.get(self),
        }
    }

    /// Resolves the value of a [`Style`].
    pub fn style<T: Clone + Default + Value>(&self, style: &Style<T>) -> T {
        match self.try_style(style) {
            Ok(value) => value,
            Err(err) => {
                tracing::warn!("{}", err);
                T::default()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_F32: Key<f32> = Key::new("test-f32");

    #[test]
    fn test_theme() {
        let mut theme = Theme::new();

        theme.set(TEST_F32, 1.0);

        assert_eq!(theme.get(TEST_F32), 1.0);
    }
}
