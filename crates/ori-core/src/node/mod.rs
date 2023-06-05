mod downcast;
mod element;
mod root;
mod state;

pub use downcast::*;
pub use element::*;
pub use state::*;

use std::{any::Any, fmt::Debug, sync::Arc};

use glam::Vec2;
use ori_graphics::Rect;
use ori_reactive::Event;
use ori_style::{FromStyleAttribute, StyleTags};
use parking_lot::{Mutex, MutexGuard};
use tracing::trace_span;

use crate::{
    AnyElement, AvailableSpace, Context, DebugEvent, DrawContext, EmptyElement, EventContext,
    LayoutContext, Margin, Padding, PointerEvent,
};

struct NodeInner<T: NodeElement> {
    view_state: Mutex<T::State>,
    node_state: Mutex<NodeState>,
    element: Mutex<T>,
}

impl<T: NodeElement> Debug for NodeInner<T>
where
    T: Debug,
    T::State: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElementInner")
            .field("view_state", &self.view_state)
            .field("node_state", &self.node_state)
            .field("view", &self.element.type_id())
            .finish()
    }
}

/// An node in the UI tree.
pub struct Node<T: NodeElement = Box<dyn AnyElement>> {
    inner: Arc<NodeInner<T>>,
}

impl<T: NodeElement> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: NodeElement> Debug for Node<T>
where
    T: Debug,
    T::State: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Node {
    /// Creates an empty [`Node`].
    pub fn empty() -> Self {
        Self::new(Box::new(EmptyElement))
    }
}

impl<T: NodeElement> Node<T> {
    /// Create a new element with the given [`Element`](crate::Element).
    pub fn new(element: T) -> Self {
        let view_state = NodeElement::build(&element);
        let element_state = NodeState::new(NodeElement::style(&element));

        let inner = Arc::new(NodeInner {
            view_state: Mutex::new(view_state),
            node_state: Mutex::new(element_state),
            element: Mutex::new(element),
        });

        Self { inner }
    }

    /// Returns a [`MutexGuard`] to the state of the `T`.
    ///
    /// Be careful when using this, as it can cause deadlocks.
    pub fn element_state(&self) -> MutexGuard<'_, T::State> {
        self.inner.as_ref().view_state.lock()
    }

    /// Returns a [`MutexGuard`] to the [`NodeState`].
    ///
    /// Be careful when using this, as it can cause deadlocks.
    pub fn node_state(&self) -> MutexGuard<'_, NodeState> {
        self.inner.as_ref().node_state.lock()
    }

    /// Returns a [`MutexGuard`] to the `T`.
    pub fn element(&self) -> MutexGuard<'_, T> {
        self.inner.as_ref().element.lock()
    }

    /// Downcasts `T` to `U` and calls the given function with the `U`.
    pub fn downcast<U: NodeElement, V>(
        &self,
        f: impl FnOnce(&mut U) -> V,
    ) -> Result<V, ElementDowncastError>
    where
        T: DowncastElement<U>,
    {
        let result = if let Some(view) = self.element().downcast_mut() {
            f(view)
        } else {
            return Err(ElementDowncastError);
        };

        self.request_layout();

        Ok(result)
    }

    /// Sets the offset of the element, relative to the parent.
    pub fn set_offset(&self, offset: Vec2) {
        let mut element_state = self.node_state();

        let size = element_state.local_rect.size();
        let min = element_state.margin.top_left() + offset;
        element_state.local_rect = Rect::min_size(min, size);
    }

    /// Get the style of the element, for a given key.
    pub fn get_style<S: FromStyleAttribute + 'static>(
        &self,
        cx: &mut impl Context,
        key: &str,
    ) -> Option<S> {
        self.node_state().get_style(cx, key)
    }

    /// Get the style of the element, for a given key. If the style is not found, `S::default()` is returned.
    pub fn style<S: FromStyleAttribute + Default + 'static>(
        &self,
        cx: &mut impl Context,
        key: &str,
    ) -> S {
        self.get_style(cx, key).unwrap_or_default()
    }

    /// Get the style of the element, for a group of keys. If the style is not found, `S::default()` is returned.
    pub fn style_group<S: FromStyleAttribute + Default + 'static>(
        &self,
        cx: &mut impl Context,
        key: &[&str],
    ) -> S {
        self.node_state().style_group(cx, key)
    }

    /// Returns the [`StyleTags`].
    pub fn style_tags(&self) -> StyleTags {
        self.node_state().style_tags()
    }

    /// Returns true if the element needs to be laid out.
    pub fn needs_layout(&self) -> bool {
        self.node_state().needs_layout
    }

    /// Returns the available space for the element.
    pub fn available_space(&self) -> AvailableSpace {
        self.node_state().available_space
    }

    /// Sets the available space for the element.
    pub fn set_available_space(&self, space: AvailableSpace) {
        self.node_state().available_space = space;
    }

    /// Returns true if the available space for the element has changed.
    pub fn space_changed(&self, space: AvailableSpace) -> bool {
        self.node_state().space_changed(space)
    }

    /// Requests a layout.
    pub fn request_layout(&self) {
        self.node_state().needs_layout = true;
    }

    /// Gets the local [`Rect`] of the element.
    pub fn local_rect(&self) -> Rect {
        self.node_state().local_rect
    }

    /// Gets the global [`Rect`] of the element.
    pub fn global_rect(&self) -> Rect {
        self.node_state().global_rect
    }

    /// Gets the size of the element.
    pub fn size(&self) -> Vec2 {
        let element_state = self.node_state();
        element_state.local_rect.size() + element_state.margin.size()
    }
}

impl<T: NodeElement> Node<T> {
    // returns true if the element should be redrawn.
    fn handle_pointer_event(
        element_state: &mut NodeState,
        event: &PointerEvent,
        is_handled: bool,
    ) -> bool {
        let contains = element_state.global_rect.contains(event.position);
        let is_over = contains && !event.left && !is_handled;
        if is_over != element_state.hovered && event.is_motion() {
            element_state.hovered = is_over;
            true
        } else {
            false
        }
    }

    // updates the cursor of the window.
    fn update_cursor(cx: &mut impl Context) {
        let Some(cursor) = cx.style("cursor") else {
            return;
        };

        if cx.hovered() || cx.active() {
            cx.window_mut().cursor = cursor;
        }
    }

    fn with_inner<C: Context, O>(
        &self,
        cx: &mut C,
        f: impl FnOnce(&mut NodeState, &mut C) -> O,
    ) -> O {
        let element_state = &mut self.node_state();
        element_state.propagate_up(cx.state_mut());

        let _span = trace_span!("element", selector = %element_state.selector()).entered();

        if element_state.needs_layout {
            cx.request_redraw();
        }

        cx.style_tree_mut().push(element_state.selector());

        let res = f(element_state, cx);

        cx.style_tree_mut().pop();

        cx.state_mut().propagate_down(element_state);

        res
    }

    fn event_inner(&self, state: &mut NodeState, cx: &mut EventContext, event: &Event) {
        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if Self::handle_pointer_event(state, pointer_event, event.is_handled()) {
                cx.request_redraw();
            }
        }

        let mut cx = EventContext {
            state,
            renderer: cx.renderer,
            window: cx.window,
            fonts: cx.fonts,
            stylesheet: cx.stylesheet,
            style_tree: cx.style_tree,
            style_cache: cx.style_cache,
            event_sink: cx.event_sink,
            image_cache: cx.image_cache,
        };

        if let Some(event) = event.get::<DebugEvent>() {
            event.with_element(&mut cx, self);
            return;
        }

        self.element()
            .event(&mut self.element_state(), &mut cx, event);

        Self::update_cursor(&mut cx);
    }

    /// Handle an event.
    pub fn event(&self, cx: &mut EventContext, event: &Event) {
        self.with_inner(cx, |element_state, cx| {
            self.event_inner(element_state, cx, event);
        });
    }

    /// Layout the element.
    pub fn layout(&self, cx: &mut LayoutContext, space: AvailableSpace) -> Vec2 {
        let size = self.relayout(cx, space);
        self.set_available_space(space);
        size
    }

    fn relayout_inner(
        &self,
        state: &mut NodeState,
        cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        state.needs_layout = false;

        let mut cx = LayoutContext {
            state,
            renderer: cx.renderer,
            window: cx.window,
            fonts: cx.fonts,
            stylesheet: cx.stylesheet,
            style_tree: cx.style_tree,
            style_cache: cx.style_cache,
            event_sink: cx.event_sink,
            image_cache: cx.image_cache,
            parent_space: cx.space,
            space,
        };

        cx.state.margin = Margin::from_style(&mut cx, space);
        cx.state.padding = Padding::from_style(&mut cx, space);

        let space = space.apply_margin(cx.state.margin);
        let space = cx.style_constraints(space);
        cx.space = space;

        let size = self
            .element()
            .layout(&mut self.element_state(), &mut cx, space);

        let local_offset = state.local_rect.min + state.margin.top_left();
        let global_offset = state.global_rect.min + state.margin.top_left();
        state.local_rect = Rect::min_size(local_offset, size);
        state.global_rect = Rect::min_size(global_offset, size);

        size + state.margin.size()
    }

    /// Relayout the element.
    ///
    /// This should be called when the element needs to be relayouted, for example when the
    /// when flex layout has left over space, and flex elements need to fill that space.
    ///
    /// For more context see the implementation of [`Children::flex_layout`](crate::Children::flex_layout).
    pub fn relayout(&self, cx: &mut LayoutContext, space: AvailableSpace) -> Vec2 {
        self.with_inner(cx, |element_state, cx| {
            self.relayout_inner(element_state, cx, space)
        })
    }

    fn draw_inner(&self, state: &mut NodeState, cx: &mut DrawContext) {
        let mut cx = DrawContext {
            state,
            frame: cx.frame,
            renderer: cx.renderer,
            window: cx.window,
            fonts: cx.fonts,
            stylesheet: cx.stylesheet,
            style_tree: cx.style_tree,
            style_cache: cx.style_cache,
            event_sink: cx.event_sink,
            image_cache: cx.image_cache,
        };

        self.element().draw(&mut self.element_state(), &mut cx);

        if cx.state.update_transitions() {
            cx.request_redraw();
            cx.request_layout();
        }

        cx.state.draw();

        Self::update_cursor(&mut cx);
    }

    /// Draw the element.
    pub fn draw(&self, cx: &mut DrawContext) {
        self.with_inner(cx, |element_state, cx| {
            self.draw_inner(element_state, cx);
        });
    }
}
