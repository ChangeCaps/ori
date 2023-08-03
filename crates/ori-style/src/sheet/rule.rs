use std::{fmt::Display, sync::Arc};

use crate::{StyleAttributeKey, StyleAttributeValue, StyleSelector, StyleTransition, StyleTree};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StyleRuleAttributeValue {
    Value(StyleAttributeValue),
    Variable(String),
    Inherit,
}

impl Display for StyleRuleAttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleRuleAttributeValue::Value(value) => write!(f, "{}", value),
            StyleRuleAttributeValue::Variable(name) => write!(f, "var({})", name),
            StyleRuleAttributeValue::Inherit => write!(f, "inherit"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StyleRuleAttribute {
    pub key: StyleAttributeKey,
    pub value: StyleRuleAttributeValue,
    pub transition: Option<StyleTransition>,
}

/// A stylesheet rule.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StyleRule {
    /// The selectors that match this rule.
    pub selectors: Arc<[StyleSelector]>,
    /// The attributes that are set by this rule.
    pub attributes: Arc<[StyleRuleAttribute]>,
}

impl StyleRule {
    pub fn get_match(&self, selector: &StyleTree) -> Option<usize> {
        self.selectors.iter().position(|s| s.matches(selector))
    }

    pub fn get_attribute(&self, key: &str) -> Option<&StyleRuleAttribute> {
        self.attributes.iter().rev().find(|a| a.key == key)
    }
}

impl Display for StyleRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let selectors = self
            .selectors
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(f, "{} {{", selectors)?;

        for attribute in &*self.attributes {
            writeln!(f, "    {}: {};", attribute.key, attribute.value)?;
        }

        writeln!(f, "}}")
    }
}
