use std::time::Instant;

use ori_graphics::Rect;
use ori_style::{
    FromStyleAttribute, Style, StyleAttribute, StyleSpec, StyleTransition, StyleTransitionStates,
};
use uuid::Uuid;

use crate::{AvailableSpace, Context, Margin, Padding};

/// An element identifier. This uses a UUID to ensure that elements are unique.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ElementId {
    uuid: Uuid,
}

impl ElementId {
    /// Create a new element identifier, using uuid v4.
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }

    /// Gets the inner uuid.
    pub const fn uuid(self) -> Uuid {
        self.uuid
    }
}

impl Default for ElementId {
    fn default() -> Self {
        Self::new()
    }
}

/// The state of a element, which is used to store information about the element.
///
/// This should almost never be used directly, and instead should be used through
/// the [`Element`](crate::Element) struct.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct NodeState {
    /// The identifier of the element.
    pub id: ElementId,
    /// The margin of the element.
    pub margin: Margin,
    /// The padding of the element.
    pub padding: Padding,
    /// The local rect of the element, relative to the parent.
    pub local_rect: Rect,
    /// The global rect of the element, relative to the window.
    pub global_rect: Rect,
    /// Whether the element is active.
    pub active: bool,
    /// Whether the element is focused.
    pub focused: bool,
    /// Whether the element is hovered.
    pub hovered: bool,
    /// Whether the element is unique.
    pub unique: bool,
    /// The last time the element was drawn.
    pub last_draw: Instant,
    /// The style of the element.
    pub style: Style,
    /// Whether the element needs to be laid out.
    pub needs_layout: bool,
    /// The available space for the element.
    pub available_space: AvailableSpace,
    /// The style transition states of the element.
    pub transitions: StyleTransitionStates,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            id: ElementId::new(),
            margin: Margin::ZERO,
            padding: Padding::ZERO,
            local_rect: Rect::ZERO,
            global_rect: Rect::ZERO,
            active: false,
            focused: false,
            hovered: false,
            unique: false,
            last_draw: Instant::now(),
            style: Style::default(),
            needs_layout: true,
            available_space: AvailableSpace::ZERO,
            transitions: StyleTransitionStates::new(),
        }
    }
}

impl NodeState {
    /// Create a new [`NodeState`] with the given style.
    pub fn new(style: Style) -> Self {
        Self {
            style,
            ..Default::default()
        }
    }

    /// Propagate the [`NodeState`] up to the parent.
    ///
    /// This is called before events are propagated.
    pub fn propagate_up(&mut self, parent: &mut NodeState) {
        self.global_rect = self.local_rect.translate(parent.global_rect.min);
    }

    /// Propagate the [`NodeState`] down to the child.
    ///
    /// This is called after events are propagated.
    pub fn propagate_down(&mut self, child: &mut NodeState) {
        self.needs_layout |= child.needs_layout;
        self.unique &= !child.hovered;
    }

    /// Updates the style tags.
    pub fn update_style_tags(&mut self) {
        self.style.tags.clear();

        if self.active {
            self.style.tags.insert("active");
        } else {
            self.style.tags.remove("active");
        }

        if self.focused {
            self.style.tags.insert("focus");
        } else {
            self.style.tags.remove("focus");
        }

        if self.hovered {
            self.style.tags.insert("hover");
        } else {
            self.style.tags.remove("hover");
        }
    }

    /// Returns the time in seconds since the last draw.
    pub fn delta_time(&self) -> f32 {
        self.last_draw.elapsed().as_secs_f32()
    }

    /// Gets the style attribute for the given key.
    pub fn get_style_attribute(
        &mut self,
        cx: &mut impl Context,
        key: &str,
    ) -> Option<StyleAttribute> {
        self.query_style_attribute(cx, key)
            .map(|(attribute, _)| attribute)
    }

    /// Gets the style attribute and specificity for the given key.
    pub fn query_style_attribute(
        &mut self,
        cx: &mut impl Context,
        key: &str,
    ) -> Option<(StyleAttribute, StyleSpec)> {
        let mut style_tree = cx.style_tree().clone();
        style_tree.push(self.style.clone());

        let (sheet, cache) = cx.stylesheet_and_cache_mut();
        sheet.query_cached(cache, None, &style_tree, key)
    }

    /// Gets the style attribute for the given key, and converts it to the given type.
    pub fn query_style<T: FromStyleAttribute + 'static>(
        &mut self,
        cx: &mut impl Context,
        key: &str,
    ) -> Option<(T, StyleSpec)> {
        let (attribute, specificity) = self.query_style_attribute(cx, key)?;
        let value = T::from_attribute(attribute.value().clone())?;
        let transition = attribute.transition();

        Some((self.transition(key, value, transition), specificity))
    }

    /// Gets the style attribute for the given key, and converts it to the given type.
    pub fn get_style<T: FromStyleAttribute + 'static>(
        &mut self,
        cx: &mut impl Context,
        key: &str,
    ) -> Option<T> {
        self.query_style(cx, key).map(|(value, _)| value)
    }

    /// Gets the style attribute for the given key, and converts it to the given type.
    pub fn style<T: FromStyleAttribute + Default + 'static>(
        &mut self,
        cx: &mut impl Context,
        key: &str,
    ) -> T {
        self.get_style(cx, key).unwrap_or_default()
    }

    /// Gets the style for a group of keys.
    pub fn style_group<T: FromStyleAttribute + Default + 'static>(
        &mut self,
        cx: &mut impl Context,
        keys: &[&str],
    ) -> T {
        let mut specificity = None;
        let mut result = None;

        for key in keys {
            if let Some((v, s)) = self.query_style(cx, key) {
                if specificity.is_none() || s > specificity.unwrap() {
                    specificity = Some(s);
                    result = Some(v);
                }
            }
        }

        result.unwrap_or_default()
    }

    /// Transition a value.
    ///
    /// If the value is an [`f32`], or a [`Color`](ori_graphics::Color), then it will be transitioned.
    pub fn transition<T: 'static>(
        &mut self,
        name: &str,
        value: T,
        transition: Option<StyleTransition>,
    ) -> T {
        self.transitions.transition_any(name, value, transition)
    }

    /// Update the transitions.
    pub fn update_transitions(&mut self) -> bool {
        self.transitions.update(self.delta_time())
    }

    /// Returns `true` if the available space has changed.
    pub fn space_changed(&mut self, space: AvailableSpace) -> bool {
        self.available_space != space
    }

    /// Updates `self.last_draw` to the current time.
    pub(crate) fn draw(&mut self) {
        self.last_draw = Instant::now();
    }
}
