use std::fmt::Display;

use crate::{
    StyleAttribute, StyleAttributeBuilder, StyleAttributes, StyleClass, StyleClasses, StyleTags,
};

/// Styling for a single element.
#[derive(Clone, Debug, Default)]
pub struct Style {
    /// The element name.
    pub element: Option<&'static str>,
    /// The classes to apply.
    pub classes: StyleClasses,
    /// The tags to apply.
    pub tags: StyleTags,
    /// The inline attributes.
    pub inline: StyleAttributes,
}

impl Style {
    /// Creates a new style with the given `element` name.
    pub const fn new(element: &'static str) -> Self {
        Self {
            element: Some(element),
            classes: StyleClasses::new(),
            tags: StyleTags::new(),
            inline: StyleAttributes::new(),
        }
    }

    /// Sets the element name.
    pub fn set_element(&mut self, element: &'static str) {
        self.element = Some(element);
    }

    /// Sets the classes.
    pub fn set_class(&mut self, class: &str) {
        let classes = class.split_whitespace().map(StyleClass::from);
        self.classes.clear();
        self.classes.extend(classes);
    }

    /// Sets the classes.
    pub fn set_attr(&mut self, key: &str, builder: impl StyleAttributeBuilder) {
        let attr = builder.attribute(key);
        self.inline.set(attr);
    }

    /// Sets the element name.
    pub fn with_element(mut self, element: &'static str) -> Self {
        self.element = Some(element);
        self
    }

    /// Adds classes to the style.
    pub fn with_class(mut self, class: &str) -> Self {
        let classes = class.split_whitespace().map(StyleClass::from);
        self.classes.extend(classes);
        self
    }

    /// Adds classes to the style.
    pub fn with_classes(
        mut self,
        classes: impl IntoIterator<Item = impl Into<StyleClass>>,
    ) -> Self {
        self.classes.extend(classes.into_iter().map(Into::into));
        self
    }

    /// Adds attributes to the style.
    pub fn with_attr(mut self, key: &str, builder: impl StyleAttributeBuilder) -> Self {
        let attr = builder.attribute(key);
        self.inline.push(attr);
        self
    }

    /// Adds attributes to the style.
    pub fn with_attrs(mut self, attrs: impl IntoIterator<Item = StyleAttribute>) -> Self {
        self.inline.extend(attrs);
        self
    }

    /// Adds a tag to the style.
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag);
        self
    }

    /// Gets the attribute with the given `key`.
    pub fn get_attribute(&self, key: &str) -> Option<&StyleAttribute> {
        self.inline.get(key)
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(element) = &self.element {
            write!(f, "{}", element)?;
        } else {
            write!(f, "*")?;
        }

        write!(f, "{}", self.classes)?;
        write!(f, "{}", self.tags)?;

        if !self.inline.is_empty() {
            write!(f, " style=\"")?;
            for (i, attr) in self.inline.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", attr)?;
            }
            write!(f, "\"")?;
        }

        Ok(())
    }
}
