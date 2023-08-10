use std::{any::Any, collections::HashMap, hash::BuildHasher, sync::Arc};

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

#[derive(Clone, Debug, Default)]
pub struct Theme {
    values: HashMap<ArcStr, Style<ArcValue>, ThemeHasher>,
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

        self.values.insert(name, value);
    }

    pub fn set_key<T: Value>(&mut self, key: Key<T>, value: Key<T>) {
        let name = ArcStr::from(key.name());
        let value: Style<ArcValue> = Style::Key(value.cast());

        self.values.insert(name, value);
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

    pub fn extend(&mut self, other: Self) {
        self.values.extend(other.values);
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn try_get<T: Clone + Value>(&self, mut key: Key<T>) -> Option<T> {
        let mut depth = 0;

        while let Some(value) = self.values.get(key.name()) {
            if depth >= Self::MAX_DEPTH {
                panic!("maximum theme depth exceeded ({}), this is likely because of a cyclic dependency", Self::MAX_DEPTH);
            }

            depth += 1;

            match value {
                Style::Concrete(value) => return value.downcast_ref::<T>().cloned(),
                Style::Key(other) => key = other.cast(),
            }
        }

        None
    }

    pub fn get<T: Clone + Default + Value>(&self, key: Key<T>) -> T {
        self.try_get(key).unwrap_or_default()
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
