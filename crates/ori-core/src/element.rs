use std::any::{self, Any, TypeId};

use glam::Vec2;
use ori_reactive::Event;
use ori_style::{Style, Styled};

use crate::{AvailableSpace, DrawContext, EventContext, LayoutContext};

/// A [`Element`] is an element in the UI tree.
///
/// # Example
/// ```
/// # use ori_core::*;
/// # use glam::*;
/// # use ori_reactive::Event;
/// # use ori_style::Style;
/// struct Foo;
///
/// impl Element for Foo {
///     // The state of the element.
///     type State = i32;
///
///     // Builds the state of the element.
///     fn build(&self) -> Self::State {
///         0
///     }
///
///     // Returns the style of the element.
///     fn style(&self) -> Style {
///         Style::new("foo")
///     }
///
///     // Handle events.
///     fn event(&self, state: &mut Self::State, cx: &mut EventContext, event: &Event) {
///         if let Some(pointer_event) = event.get::<PointerEvent>() {
///             if pointer_event.is_pressed(PointerButton::Primary) {
///                 *state += 1;
///                 cx.request_layout();
///             }
///         }
///     }
///
///     // Handle layout and returns the size of the element.
///     fn layout(
///         &self,
///         state: &mut Self::State,
///         cx: &mut LayoutContext,
///         space: AvailableSpace,
///     ) -> Vec2 {
///         space.max / *state as f32
///     }
///
///     // Draws the element.
///     fn draw(&self, _state: &mut Self::State, cx: &mut DrawContext) {
///         cx.draw_quad();
///     }
/// }
/// ```
#[allow(unused_variables)]
pub trait Element: Send + Sync + 'static {
    /// The state of the element.
    type State: Send + Sync + 'static;

    /// Builds the state of the element.
    fn build(&self) -> Self::State;

    /// Returns the style of the element.
    fn style(&self) -> Style {
        Style::default()
    }

    /// Handles an event.
    fn event(&self, state: &mut Self::State, cx: &mut EventContext, event: &Event) {}

    /// Handle layout and returns the size of the element.
    ///
    /// This method should return a size that fits the [`AvailableSpace`].
    ///
    /// The default implementation returns the minimum size.
    fn layout(
        &self,
        state: &mut Self::State,
        cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        space.min
    }

    /// Draws the view.
    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext) {}
}

/// A type-erased [`Element`].
pub trait AnyElement: Any + Send + Sync {
    /// Builds the state of the view.
    fn build(&self) -> Box<dyn Any + Send + Sync>;

    /// Returns the style of the view.
    fn style(&self) -> Style;

    /// Handles an event.
    fn event(&self, state: &mut dyn Any, cx: &mut EventContext, event: &Event);

    /// Layout the view.
    fn layout(&self, state: &mut dyn Any, cx: &mut LayoutContext, space: AvailableSpace) -> Vec2;

    /// Draws the view.
    fn draw(&self, state: &mut dyn Any, cx: &mut DrawContext);
}

impl<T: Element> AnyElement for T {
    fn build(&self) -> Box<dyn Any + Send + Sync> {
        Box::new(self.build())
    }

    fn style(&self) -> Style {
        self.style()
    }

    fn event(&self, state: &mut dyn Any, cx: &mut EventContext, event: &Event) {
        if let Some(state) = state.downcast_mut::<T::State>() {
            self.event(state, cx, event);
        } else {
            tracing::warn!("invalid state type on {}", any::type_name::<T>());
        }
    }

    fn layout(&self, state: &mut dyn Any, cx: &mut LayoutContext, space: AvailableSpace) -> Vec2 {
        if let Some(state) = state.downcast_mut::<T::State>() {
            self.layout(state, cx, space)
        } else {
            tracing::warn!("invalid state type on {}", any::type_name::<T>());
            space.min
        }
    }

    fn draw(&self, state: &mut dyn Any, cx: &mut DrawContext) {
        if let Some(state) = state.downcast_mut::<T::State>() {
            self.draw(state, cx);
        } else {
            tracing::warn!("invalid state type on {}", any::type_name::<T>());
        }
    }
}

impl dyn AnyElement {
    pub fn downcast_ref<T: AnyElement>(&self) -> Option<&T> {
        if self.type_id() == TypeId::of::<T>() {
            // SAFETY: `T` and `Self` are the same type
            unsafe { Some(&*(self as *const dyn AnyElement as *const T)) }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: AnyElement>(&mut self) -> Option<&mut T> {
        if <dyn AnyElement>::type_id(self) == TypeId::of::<T>() {
            // SAFETY: `T` and `Self` are the same type
            unsafe { Some(&mut *(self as *mut dyn AnyElement as *mut T)) }
        } else {
            None
        }
    }
}

impl<V: Element> Element for Styled<V> {
    type State = V::State;

    fn build(&self) -> Self::State {
        self.value.build()
    }

    fn style(&self) -> Style {
        let mut style = self.value.style();
        style.classes.extend(self.classes.clone());
        style.attributes.extend(self.attributes.clone());
        style
    }

    fn event(&self, state: &mut Self::State, cx: &mut EventContext, event: &Event) {
        self.value.event(state, cx, event)
    }

    fn layout(
        &self,
        state: &mut Self::State,
        cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        self.value.layout(state, cx, space)
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext) {
        self.value.draw(state, cx)
    }
}

/// A [`Element`] that does nothing.
#[derive(Clone, Copy, Debug, Default)]
pub struct EmptyElement;

impl Element for EmptyElement {
    type State = ();

    fn build(&self) -> Self::State {}
}
