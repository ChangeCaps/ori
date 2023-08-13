use std::{
    any::Any,
    collections::HashMap,
    fmt::{Debug, Display},
    hash::BuildHasher,
    sync::Arc,
};

use crate::{Key, Style};

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

#[derive(Clone)]
enum ThemeEntry {
    Style(Style<ArcValue>),
    Map {
        key: ArcStr,
        map: Arc<dyn Fn(&ArcValue) -> ArcValue + Send + Sync>,
    },
}

impl Debug for ThemeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeEntry::Style(style) => f.debug_tuple("Style").field(style).finish(),
            ThemeEntry::Map { key, .. } => f.debug_tuple("Map").field(key).finish(),
        }
    }
}

#[derive(Debug)]
pub enum ThemeGetError {
    NotFound(ArcStr),
    MaxDepth,
    Downcast,
}

impl Display for ThemeGetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeGetError::NotFound(name) => write!(f, "missing theme value for key '{}'", name),
            ThemeGetError::MaxDepth => write!(f, "maximum theme depth exceeded"),
            ThemeGetError::Downcast => write!(f, "failed to downcast theme value"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Theme {
    values: HashMap<ArcStr, ThemeEntry, ThemeHasher>,
}

impl Theme {
    const MAX_DEPTH: usize = 32;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn set_arc<T: Value>(&mut self, key: Key<T>, value: Arc<T>) {
        let name = ArcStr::from(key.name());
        let value: Style<ArcValue> = Style::Concrete(value);

        self.values.insert(name, ThemeEntry::Style(value));
    }

    pub fn set_key<T: Value>(&mut self, key: Key<T>, value: Key<T>) {
        let name = ArcStr::from(key.name());
        let value: Style<ArcValue> = Style::Key(value.cast());

        self.values.insert(name, ThemeEntry::Style(value));
    }

    pub fn set<T: Value>(&mut self, key: Key<T>, value: impl Into<Style<T>>) {
        match value.into() {
            Style::Concrete(value) => {
                self.set_arc(key, Arc::new(value));
            }
            Style::Key(value) => {
                self.set_key(key, value);
            }
        }
    }

    pub fn map<T: Value, U: Value>(
        &mut self,
        key: Key<U>,
        source: Key<T>,
        map: impl Fn(&T) -> U + Send + Sync + 'static,
    ) {
        let name = ArcStr::from(key.name());
        let map = Arc::new(move |value: &ArcValue| {
            let value = value.downcast_ref::<T>().unwrap();
            Arc::new(map(value)) as ArcValue
        });

        self.values.insert(
            name,
            ThemeEntry::Map {
                key: ArcStr::from(source.name()),
                map,
            },
        );
    }

    pub fn extend(&mut self, other: Self) {
        self.values.extend(other.values);
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    fn try_get_value(&self, mut key: &str) -> Result<ArcValue, ThemeGetError> {
        let mut depth = 0;

        while let Some(value) = self.values.get(key) {
            if depth >= Self::MAX_DEPTH {
                return Err(ThemeGetError::MaxDepth);
            }

            depth += 1;

            match value {
                ThemeEntry::Style(style) => match style {
                    Style::Concrete(value) => return Ok(value.clone()),
                    Style::Key(other) => key = other.name(),
                },
                ThemeEntry::Map { key, map } => {
                    let value = self.try_get_value(key)?;
                    return Ok(map(&value));
                }
            }
        }

        Err(ThemeGetError::NotFound(ArcStr::from(key)))
    }

    pub fn try_get<T: Clone + Value>(&self, key: Key<T>) -> Result<T, ThemeGetError> {
        let value = self.try_get_value(key.name())?;
        (value.downcast_ref::<T>().cloned()).ok_or(ThemeGetError::Downcast)
    }

    pub fn get<T: Clone + Default + Value>(&self, key: Key<T>) -> T {
        match self.try_get(key) {
            Ok(value) => value,
            Err(err) => {
                tracing::warn!("{}", err);
                T::default()
            }
        }
    }

    pub fn style<T: Clone + Default + Value>(&self, style: &Style<T>) -> T {
        match style {
            Style::Concrete(value) => value.clone(),
            Style::Key(key) => self.get(*key),
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
