mod build;
mod r#ref;
mod root;
mod state;

pub use build::*;
pub use r#ref::*;
pub use state::*;

use std::{any::Any, fmt::Debug, sync::Arc};

use glam::Vec2;
use ori_graphics::Rect;
use ori_reactive::Event;
use ori_style::FromStyleAttribute;
use parking_lot::{Mutex, MutexGuard};

use crate::{
    AnyElement, AvailableSpace, Build, Context, DebugEvent, DrawContext, Element, EmptyElement,
    EventContext, ForceLayoutEvent, LayoutContext, Margin, Padding, PointerEvent,
};

struct NodeInner {
    view_state: Mutex<Box<dyn Any + Send>>,
    node_state: Mutex<Box<NodeState>>,
    element: Mutex<Box<dyn AnyElement>>,
}

impl Debug for NodeInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeInner")
            .field("view_state", &self.view_state)
            .field("node_state", &self.node_state)
            .field("view", &self.element.type_id())
            .finish()
    }
}

/// An error that occurs when downcasting an element.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ElementDowncastError;

/// An node in the UI tree.
#[derive(Clone)]
pub struct Node {
    inner: Arc<NodeInner>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Node {
    /// Creates an empty [`Node`].
    pub fn empty() -> Self {
        Self::new(EmptyElement)
    }

    /// Create a new element with the given [`Element`](crate::Element).
    pub fn new(element: impl Element) -> Self {
        let view_state = Element::build(&element);
        let element_state = NodeState::new(Element::style(&element));

        let inner = Arc::new(NodeInner {
            view_state: Mutex::new(Box::new(view_state)),
            node_state: Mutex::new(Box::new(element_state)),
            element: Mutex::new(Box::new(element)),
        });

        Self { inner }
    }

    /// Returns a [`MutexGuard`] to the state of the `T`.
    ///
    /// Be careful when using this, as it can cause deadlocks.
    pub fn element_state(&self) -> MutexGuard<'_, Box<dyn Any + Send>> {
        self.inner.as_ref().view_state.lock()
    }

    /// Returns a [`MutexGuard`] to the [`NodeState`].
    ///
    /// Be careful when using this, as it can cause deadlocks.
    pub fn node_state(&self) -> MutexGuard<'_, Box<NodeState>> {
        self.inner.as_ref().node_state.lock()
    }

    /// Returns a [`MutexGuard`] to the `T`.
    pub fn element(&self) -> MutexGuard<'_, Box<dyn AnyElement>> {
        self.inner.as_ref().element.lock()
    }

    /// Downcasts `T` to `U` and calls the given function with the `U`.
    pub fn downcast<U: Element, V>(
        &self,
        f: impl FnOnce(&mut U) -> V,
    ) -> Result<V, ElementDowncastError> {
        let result = if let Some(view) = self.element().downcast_mut() {
            f(view)
        } else {
            return Err(ElementDowncastError);
        };

        self.request_layout();

        Ok(result)
    }

    /// Returns the [`PropGuard`] for the given [`Element`].
    pub fn try_prop<B: Element + Build>(&self) -> Option<PropGuard<'_, B>> {
        let element = self.element();

        if !<dyn AnyElement>::is::<B>(element.as_ref()) {
            return None;
        }

        // SAFETY: we just checked that the element is of type `B`.
        unsafe { Some(PropGuard::new(element)) }
    }

    /// Returns the [`PropGuard`] for the given [`Element`].
    #[track_caller]
    pub fn prop<B: Element + Build>(&self) -> PropGuard<'_, B> {
        match self.try_prop() {
            Some(prop) => prop,
            None => panic!("Element is not of type {}", std::any::type_name::<B>()),
        }
    }

    /// Returns the [`OnGuard`] for the given [`Element`].
    pub fn try_on<B: Element + Build>(&self) -> Option<OnGuard<'_, B>> {
        let element = self.element();

        if !<dyn AnyElement>::is::<B>(element.as_ref()) {
            return None;
        }

        // SAFETY: we just checked that the element is of type `B`.
        unsafe { Some(OnGuard::new(element)) }
    }

    /// Returns the [`OnGuard`] for the given [`Element`].
    #[track_caller]
    pub fn on<B: Element + Build>(&self) -> OnGuard<'_, B> {
        match self.try_on() {
            Some(on) => on,
            None => panic!("Element is not of type {}", std::any::type_name::<B>()),
        }
    }

    /// Returns the [`BindGuard`] for the given [`Element`].
    pub fn try_bind<B: Element + Build>(&self) -> Option<BindGuard<'_, B>> {
        let element = self.element();

        if !<dyn AnyElement>::is::<B>(element.as_ref()) {
            return None;
        }

        // SAFETY: we just checked that the element is of type `B`.
        unsafe { Some(BindGuard::new(element)) }
    }

    /// Returns the [`BindGuard`] for the given [`Element`].
    #[track_caller]
    pub fn bind<B: Element + Build>(&self) -> BindGuard<'_, B> {
        match self.try_bind() {
            Some(bind) => bind,
            None => panic!("Element is not of type {}", std::any::type_name::<B>()),
        }
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
    pub fn rect(&self) -> Rect {
        self.node_state().global_rect
    }

    /// Whether the element is hovered.
    pub fn hovered(&self) -> bool {
        self.node_state().hovered
    }

    /// Whether the element is focused.
    pub fn focused(&self) -> bool {
        self.node_state().focused
    }

    /// Whether the element is active.
    pub fn active(&self) -> bool {
        self.node_state().active
    }

    /// Gets the size of the element.
    pub fn size(&self) -> Vec2 {
        let element_state = self.node_state();
        element_state.local_rect.size() + element_state.margin.size()
    }
}

impl Node {
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
        let node_state = &mut self.node_state();
        node_state.propagate_up(cx.node_mut());

        if node_state.needs_layout {
            cx.request_redraw();
        }

        node_state.update_style_tags();
        cx.style_tree_mut().push(node_state.style.clone());
        let res = f(node_state, cx);
        cx.style_tree_mut().pop();

        cx.node_mut().propagate_down(node_state);

        res
    }

    fn event_inner(&self, state: &mut NodeState, cx: &mut EventContext, event: &Event) {
        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if Self::handle_pointer_event(state, pointer_event, event.is_handled()) {
                cx.request_layout();
            }
        }

        if event.is::<ForceLayoutEvent>() {
            state.needs_layout = true;
        }

        let mut cx = EventContext {
            node: state,
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

        (self.element()).event(self.element_state().as_mut(), &mut cx, event);

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
            node: state,
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

        cx.node.margin = Margin::from_style(&mut cx, space);
        cx.node.padding = Padding::from_style(&mut cx, space);

        let space = space.apply_margin(cx.node.margin);
        let space = cx.style_constraints(space);
        cx.space = space;

        let size = (self.element()).layout(self.element_state().as_mut(), &mut cx, space);

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
        let parent_size = cx.size();
        let mut cx = DrawContext {
            node: state,
            frame: cx.frame,
            renderer: cx.renderer,
            window: cx.window,
            fonts: cx.fonts,
            parent_size,
            stylesheet: cx.stylesheet,
            style_tree: cx.style_tree,
            style_cache: cx.style_cache,
            event_sink: cx.event_sink,
            image_cache: cx.image_cache,
        };

        self.element().draw(self.element_state().as_mut(), &mut cx);

        if cx.node.update_transitions() {
            cx.request_redraw();
            cx.request_layout();
        }

        cx.node.draw();

        Self::update_cursor(&mut cx);
    }

    /// Draw the element.
    pub fn draw(&self, cx: &mut DrawContext) {
        self.with_inner(cx, |element_state, cx| {
            self.draw_inner(element_state, cx);
        });
    }
}
