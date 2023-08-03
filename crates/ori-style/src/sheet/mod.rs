mod rule;
mod theme;

pub use rule::*;

use std::{fmt::Display, fs, io, path::Path, str::FromStr};

use crate::{
    StyleAttribute, StyleCache, StyleCacheEntry, StyleCacheKey, StyleSpec, StyleTree,
    StyleheetParseError,
};

/// An error that can occur when loading a style sheet.
#[derive(Debug)]
pub enum StyleLoadError {
    /// An error occurred while parsing the style sheet.
    Parse(StyleheetParseError),
    /// An error occurred while reading the style sheet.
    Io(io::Error),
}

impl From<StyleheetParseError> for StyleLoadError {
    fn from(error: StyleheetParseError) -> Self {
        Self::Parse(error)
    }
}

impl From<io::Error> for StyleLoadError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl Display for StyleLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(error) => write!(f, "Parse error: {}", error),
            Self::Io(error) => write!(f, "IO error: {}", error),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StyleQuery {
    pub attribute: StyleAttribute,
    pub specificity: StyleSpec,
    pub inherited: bool,
}

/// A stylesheet is a list of rules.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stylesheet {
    pub rules: Vec<StyleRule>,
}

impl Stylesheet {
    /// Creates a new empty stylesheet.
    pub fn empty() -> Self {
        Self { rules: Vec::new() }
    }

    /// Create a new stylesheet.
    pub fn new() -> Self {
        Self::from_str(theme::DEFAULT).unwrap()
    }

    /// The default day theme.
    pub fn day_theme() -> Self {
        Self::from_str(theme::DAY).unwrap()
    }

    /// The default night theme.
    pub fn night_theme() -> Self {
        Self::from_str(theme::NIGHT).unwrap()
    }

    fn load_dir(path: &Path) -> Result<Self, StyleLoadError> {
        let mut sheet = Self::empty();

        for entry in path.read_dir()? {
            let entry = entry?;
            sheet.extend(Self::load(entry.path())?);
        }

        Ok(sheet)
    }

    fn load_file(path: &Path) -> Result<Self, StyleLoadError> {
        let input = fs::read_to_string(path)?;
        Ok(Self::from_str(&input)?)
    }

    /// Loads a style sheet from a file.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, StyleLoadError> {
        if path.as_ref().is_dir() {
            Self::load_dir(path.as_ref())
        } else {
            Self::load_file(path.as_ref())
        }
    }

    /// Add a rule to the stylesheet.
    pub fn add_rule(&mut self, rule: StyleRule) {
        self.rules.push(rule);
    }

    /// Extend the stylesheet with a list of rules.
    pub fn extend(&mut self, rules: impl IntoIterator<Item = StyleRule>) {
        self.rules.extend(rules);
    }

    /// Get and attribute and its specificity from the style sheet.
    pub fn query_cached(
        &self,
        cache: &mut StyleCache,
        cache_key: Option<StyleCacheKey>,
        tree: &StyleTree,
        key: &str,
    ) -> Option<StyleQuery> {
        let (attr, spec) = self.query_cached_recurse_inner(cache, cache_key, tree, key)?;

        match attr.value {
            StyleRuleAttributeValue::Value(value) => {
                let attribute = StyleAttribute::new(attr.key, value, attr.transition);
                Some(StyleQuery {
                    attribute,
                    specificity: spec,
                    inherited: false,
                })
            }
            StyleRuleAttributeValue::Variable(var) => {
                if key == var {
                    return None;
                }

                let attribute = self.query_variable(cache, tree, &var)?;
                let value = attribute.value().clone();
                let transition = attr.transition;
                let attribute = StyleAttribute::new(key.into(), value, transition);

                Some(StyleQuery {
                    attribute,
                    specificity: spec,
                    inherited: false,
                })
            }
            StyleRuleAttributeValue::Inherit => {
                let parent = tree.parent()?;
                let query = self.query_cached(cache, None, &parent, key)?;
                Some(StyleQuery {
                    attribute: query.attribute,
                    specificity: spec,
                    inherited: true,
                })
            }
        }
    }

    fn query_variable(
        &self,
        cache: &mut StyleCache,
        tree: &StyleTree,
        key: &str,
    ) -> Option<StyleAttribute> {
        let mut tree = tree.clone();

        loop {
            if let Some(query) = self.query_cached(cache, None, &tree, key) {
                return Some(query.attribute);
            }

            tree = tree.parent()?;
        }
    }

    fn query_cached_recurse_inner(
        &self,
        cache: &mut StyleCache,
        cache_key: Option<StyleCacheKey>,
        tree: &StyleTree,
        key: &str,
    ) -> Option<(StyleRuleAttribute, StyleSpec)> {
        // first check if the attribute is inline, in that case return early
        if let Some(attribute) = tree.element.inline.get(key) {
            let attribute = StyleRuleAttribute {
                key: attribute.key().clone(),
                value: StyleRuleAttributeValue::Value(attribute.value().clone()),
                transition: attribute.transition(),
            };

            return Some((attribute, StyleSpec::INLINE));
        }

        let cache_key = match cache_key {
            Some(key) => key,
            None => tree.cache_key(),
        };

        // we need to check the cache, if the attribute was found in the cache, then we can return
        if let Some(entry) = cache.get(cache_key, key) {
            match entry {
                Some(entry) => return Some((entry.attribute.clone(), entry.specificity)),
                None => return None,
            }
        }

        // if the attribute was not found in the cache, then we need to search the stylesheet
        let Some((attr, spec)) =  self.query_attribute_specificity_inner(tree, key) else {
            cache.insert(cache_key, key, None);
            return None;
        };

        let cache_entry = StyleCacheEntry {
            attribute: attr.clone(),
            specificity: spec,
        };

        cache.insert(cache_key, key, Some(cache_entry));

        Some((attr, spec))
    }

    fn query_attribute_specificity_inner(
        &self,
        tree: &StyleTree,
        name: &str,
    ) -> Option<(StyleRuleAttribute, StyleSpec)> {
        let mut specificity = StyleSpec::default();
        let mut result = None;

        for rule in self.rules.iter() {
            let Some(selector) = rule.get_match(tree) else {
                continue;
            };

            let selector = &rule.selectors[selector];
            let spec = selector.spec();

            if spec < specificity {
                continue;
            }

            if let Some(attribute) = rule.get_attribute(name) {
                specificity = spec;
                result = Some((attribute, spec));
            }
        }

        result.map(|(attribute, _)| (attribute.clone(), specificity))
    }
}

impl IntoIterator for Stylesheet {
    type Item = StyleRule;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}

impl Display for Stylesheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rule in &self.rules {
            writeln!(f, "{}", rule)?;
        }

        Ok(())
    }
}
