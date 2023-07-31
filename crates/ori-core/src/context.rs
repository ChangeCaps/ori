use std::{
    any::Any,
    mem,
    ops::{Deref, DerefMut, Range},
};

use glam::Vec2;
use ori_graphics::{
    Fonts, Frame, Glyphs, ImageCache, ImageHandle, ImageSource, Quad, Rect, Renderer, TextSection,
};
use ori_reactive::EventSink;
use ori_style::{
    FromStyleAttribute, Length, StyleAttribute, StyleCache, StyleSpec, StyleTree, Stylesheet,
};

use crate::{AvailableSpace, Margin, NodeState, Padding, RequestRedrawEvent, Window};

/// A context for [`Element::event`](crate::Element::event).
#[allow(missing_docs)]
pub struct EventContext<'a> {
    pub node: &'a mut NodeState,
    pub renderer: &'a dyn Renderer,
    pub window: &'a mut Window,
    pub fonts: &'a mut Fonts,
    pub stylesheet: &'a Stylesheet,
    pub style_tree: &'a mut StyleTree,
    pub style_cache: &'a mut StyleCache,
    pub event_sink: &'a EventSink,
    pub image_cache: &'a mut ImageCache,
}

/// A context for [`Element::layout`](crate::Element::layout).
#[allow(missing_docs)]
pub struct LayoutContext<'a> {
    pub node: &'a mut NodeState,
    pub renderer: &'a dyn Renderer,
    pub window: &'a mut Window,
    pub fonts: &'a mut Fonts,
    pub stylesheet: &'a Stylesheet,
    pub style_tree: &'a mut StyleTree,
    pub style_cache: &'a mut StyleCache,
    pub event_sink: &'a EventSink,
    pub image_cache: &'a mut ImageCache,
    pub parent_space: AvailableSpace,
    pub space: AvailableSpace,
}

impl<'a> LayoutContext<'a> {
    /// Gets the available space, constrained by the element's style.
    pub fn style_constraints(&mut self, space: AvailableSpace) -> AvailableSpace {
        let min_width_group = &["min-width", "width", "size"];
        let max_width_group = &["max-width", "width", "size"];
        let min_height_group = &["min-height", "height", "size"];
        let max_height_group = &["max-height", "height", "size"];

        let parent = self.parent_space;
        let min_width = self.style_range_group(min_width_group, parent.x_axis());
        let max_width = self.style_range_group(max_width_group, parent.x_axis());

        let min_height = self.style_range_group(min_height_group, parent.y_axis());
        let max_height = self.style_range_group(max_height_group, parent.y_axis());

        let min_size = space.constrain(Vec2::new(min_width, min_height));
        let max_size = space.constrain(Vec2::new(max_width, max_height));

        AvailableSpace::new(min_size, max_size)
    }

    /// Calls `f`, temporarily changing the available space.
    pub fn with_space<T>(
        &mut self,
        mut space: AvailableSpace,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        mem::swap(&mut self.space, &mut space);
        let result = f(self);
        mem::swap(&mut self.space, &mut space);

        result
    }
}

/// A layer for drawing, see [`DrawContext::layer`](DrawContext::layer).
pub struct DrawLayer<'a, 'b> {
    draw_context: &'b mut DrawContext<'a>,
    z_index: f32,
    clip: Option<Rect>,
}

impl<'a, 'b> DrawLayer<'a, 'b> {
    /// Set the z-index of the layer.
    pub fn z_index(mut self, depth: f32) -> Self {
        self.z_index = depth;
        self
    }

    /// Set the clipping rectangle for the layer.
    pub fn clip(mut self, clip: Rect) -> Self {
        self.clip = Some(clip.round());
        self
    }

    /// Draw the layer.
    pub fn draw(self, f: impl FnOnce(&mut DrawContext)) {
        let layer = self
            .draw_context
            .frame
            .layer()
            .z_index(self.z_index)
            .clip(self.clip);

        layer.draw(|frame| {
            let mut child = DrawContext {
                node: self.draw_context.node,
                frame,
                renderer: self.draw_context.renderer,
                window: self.draw_context.window,
                fonts: self.draw_context.fonts,
                parent_size: self.draw_context.parent_size,
                stylesheet: self.draw_context.stylesheet,
                style_tree: self.draw_context.style_tree,
                style_cache: self.draw_context.style_cache,
                event_sink: self.draw_context.event_sink,
                image_cache: self.draw_context.image_cache,
            };

            f(&mut child);
        });
    }
}

/// A context for [`Element::draw`](crate::Element::draw).
#[allow(missing_docs)]
pub struct DrawContext<'a> {
    pub node: &'a mut NodeState,
    pub frame: &'a mut Frame,
    pub renderer: &'a dyn Renderer,
    pub window: &'a mut Window,
    pub fonts: &'a mut Fonts,
    pub parent_size: Vec2,
    pub stylesheet: &'a Stylesheet,
    pub style_tree: &'a mut StyleTree,
    pub style_cache: &'a mut StyleCache,
    pub event_sink: &'a EventSink,
    pub image_cache: &'a mut ImageCache,
}

impl<'a> DrawContext<'a> {
    /// Returns the frame.
    pub fn frame(&mut self) -> &mut Frame {
        self.frame
    }

    /// Returns a new layer, see [`Frame::layer`].
    pub fn layer<'b>(&'b mut self) -> DrawLayer<'a, 'b> {
        DrawLayer {
            draw_context: self,
            z_index: 1.0,
            clip: None,
        }
    }

    /// Runs the given callback on a new layer offset by the given amount.
    ///
    /// `offset` should almost always be `1.0`.
    pub fn draw_layer(&mut self, f: impl FnOnce(&mut DrawContext)) {
        self.layer().draw(f);
    }

    /// Draws the given text.
    pub fn draw_text(&mut self, glyphs: &Glyphs, rect: Rect) {
        if let Some(mesh) = self.fonts.text_mesh(self.renderer, glyphs, rect) {
            self.draw(mesh);
        }
    }

    /// Gets the border radius for the given element.
    ///
    /// # Arguments
    /// - `name`: The name of the border, e.g. `border` or `content-border`.
    /// - `parent_size`: The size of the parent element.
    pub fn style_border_radius(&mut self, name: &str, parent_size: Vec2) -> [f32; 4] {
        let tl = format!("{}-top-left-radius", name);
        let tr = format!("{}-top-right-radius", name);
        let bl = format!("{}-bottom-left-radius", name);
        let br = format!("{}-bottom-right-radius", name);

        let border_radius = format!("{}-radius", name);

        let tl: &[&str] = &[&tl, &border_radius];
        let tr: &[&str] = &[&tr, &border_radius];
        let bl: &[&str] = &[&bl, &border_radius];
        let br: &[&str] = &[&br, &border_radius];

        let range = 0.0..parent_size.min_element();
        let tl = self.style_range_group(tl, range.clone());
        let tr = self.style_range_group(tr, range.clone());
        let bl = self.style_range_group(bl, range.clone());
        let br = self.style_range_group(br, range);

        [tl, tr, br, bl]
    }

    /// Draws the quad at the current layout rect.
    ///
    /// This will use the following style attributes:
    /// - `background-color`: The background color of the quad.
    /// - `border-radius`: The border radius of the quad overwritten by the more specific
    /// attributes.
    /// - `border-top-left-radius`: The top left border radius of the quad.
    /// - `border-top-right-radius`: The top right border radius of the quad.
    /// - `border-bottom-right-radius`: The bottom right border radius of the quad.
    /// - `border-bottom-left-radius`: The bottom left border radius of the quad.
    /// - `border-width`: The border width of the quad.
    pub fn draw_background(&mut self) {
        let range = 0.0..self.rect().size().min_element();

        let quad = Quad {
            rect: self.rect(),
            background: self.style_group(&["background-color", "background"]),
            border_radius: self.style_border_radius("border", self.parent_size),
            border_width: self.style_range("border-width", range),
            border_color: self.style("border-color"),
        };

        self.draw(quad);
    }
}

impl<'a> Deref for DrawContext<'a> {
    type Target = Frame;

    fn deref(&self) -> &Self::Target {
        self.frame
    }
}

impl<'a> DerefMut for DrawContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.frame
    }
}

/// A context that is passed to [`View`](crate::view::View) methods.
///
/// See [`EventContext`], [`DrawContext`] and [`LayoutContext`] for more information.
pub trait Context {
    /// Returns the [`Stylesheet`] of the application.
    fn stylesheet(&self) -> &Stylesheet;

    /// Returns the [`StyleCache`] of the application.
    fn style_cache(&self) -> &StyleCache;

    /// Returns the [`StyleCache`] of the application.
    fn style_cache_mut(&mut self) -> &mut StyleCache;

    fn stylesheet_and_cache_mut(&mut self) -> (&Stylesheet, &mut StyleCache);

    /// Gets the [`StyleAttribute`] and [`StyleSpec`] for the given `key`.
    fn query_style_attribute(&mut self, key: &str) -> Option<(StyleAttribute, StyleSpec)>;

    /// Returns the [`NodeState`] of the current element.
    fn node(&self) -> &NodeState;

    /// Returns the [`NodeState`] of the current element.
    fn node_mut(&mut self) -> &mut NodeState;

    /// Returns the [`Renderer`] of the application.
    fn renderer(&self) -> &dyn Renderer;

    /// Returns the [`Window`] of the application.
    fn window(&self) -> &Window;

    /// Returns the [`Window`] of the application.
    fn window_mut(&mut self) -> &mut Window;

    /// Returns the [`Fonts`] of the application.
    fn fonts(&self) -> &Fonts;

    /// Returns the [`Fonts`] of the application.
    fn fonts_mut(&mut self) -> &mut Fonts;

    /// Returns the [`StyleTree`] of the current element.
    fn style_tree(&self) -> &StyleTree;

    /// Returns the [`StyleTree`] of the current element.
    fn style_tree_mut(&mut self) -> &mut StyleTree;

    /// Returns the [`EventSink`] of the application.
    fn event_sink(&self) -> &EventSink;

    /// Returns the [`ImageCache`] of the application.
    fn image_cache(&self) -> &ImageCache;

    /// Returns the [`ImageCache`] of the application.
    fn image_cache_mut(&mut self) -> &mut ImageCache;

    /// Gets the [`StyleAttribute`] for the given `key`.
    fn get_style_attribute(&mut self, key: &str) -> Option<StyleAttribute> {
        self.query_style_attribute(key)
            .map(|(attribute, _)| attribute)
    }

    /// Gets the value of a style attribute for the given `key`.
    fn query_style<T: FromStyleAttribute + 'static>(
        &mut self,
        key: &str,
    ) -> Option<(T, StyleSpec)> {
        let (attribute, specificity) = self.query_style_attribute(key)?;
        let value = T::from_attribute(attribute.value().clone())?;
        let transition = attribute.transition();

        Some((
            self.node_mut().transition(key, value, transition),
            specificity,
        ))
    }

    /// Gets the value of a style attribute for the given `key`.
    ///
    /// This will also transition the value if the attribute has a transition.
    fn get_style<T: FromStyleAttribute + 'static>(&mut self, key: &str) -> Option<T> {
        self.query_style(key).map(|(value, _)| value)
    }

    /// Gets the value of a style attribute for the given `key`, if there is no value, returns `T::default()`.
    ///
    /// This will also transition the value if the attribute has a transition.
    #[track_caller]
    fn style<T: FromStyleAttribute + Default + 'static>(&mut self, key: &str) -> T {
        self.get_style(key).unwrap_or_default()
    }

    /// Takes a `primary_key` and a `secondary_key` and returns the value of the attribute with the highest specificity.
    /// If both attributes have the same specificity, the `primary_key` will be used.
    ///
    /// This will also transition the value if the attribute has a transition.
    fn style_group<T: FromStyleAttribute + Default + 'static>(&mut self, keys: &[&str]) -> T {
        let mut specificity = None;
        let mut result = None;

        for key in keys {
            if let Some((v, s)) = self.query_style(key) {
                if specificity.is_none() || s > specificity.unwrap() {
                    specificity = Some(s);
                    result = Some(v);
                }
            }
        }

        result.unwrap_or_default()
    }

    /// Gets the value of a style attribute in pixels for the given `key`.
    /// `range` is the range from 0% to 100% of the desired value.
    ///
    /// This will also transition the value if the attribute has a transition.
    fn get_style_range(&mut self, key: &str, range: Range<f32>) -> Option<f32> {
        let attribute = self.get_style_attribute(key)?;
        let value = Length::from_attribute(attribute.value().clone())?;
        let transition = attribute.transition();

        let scale = self.window().scale;
        let width = self.window().size.x as f32;
        let height = self.window().size.y as f32;
        let pixels = value.pixels(range, scale, width, height);

        Some((self.node_mut()).transition(key, pixels, transition))
    }

    /// Gets the value of a style attribute in pixels and [`StyleSpec`] for the given `key`.
    fn get_style_range_specificity(
        &mut self,
        key: &str,
        range: Range<f32>,
    ) -> Option<(f32, StyleSpec)> {
        let (attribute, specificity) = self.query_style_attribute(key)?;
        let value = Length::from_attribute(attribute.value().clone())?;
        let transition = attribute.transition();

        let scale = self.window().scale;
        let width = self.window().size.x as f32;
        let height = self.window().size.y as f32;
        let pixels = value.pixels(range, scale, width, height);

        Some((
            (self.node_mut()).transition(key, pixels, transition),
            specificity,
        ))
    }

    /// Gets the value of a style attribute in pixels for the given `key`, if there is no value, returns `0.0`.
    /// `range` is the range from 0% to 100% of the desired value.
    ///
    /// This will also transition the value if the attribute has a transition.
    #[track_caller]
    fn style_range(&mut self, key: &str, range: Range<f32>) -> f32 {
        self.get_style_range(key, range).unwrap_or_default()
    }

    /// Takes a `primary_key` and a `secondary_key` and returns the value of the attribute with the highest specificity in pixels.
    /// If both attributes have the same specificity, the `primary_key` will be used.
    /// `range` is the range from 0% to 100% of the desired value.
    ///
    /// This will also transition the value if the attribute has a transition.
    fn style_range_group(&mut self, keys: &[&str], range: Range<f32>) -> f32 {
        let mut specificity = None;
        let mut result = None;

        for key in keys {
            if let Some((v, s)) = self.get_style_range_specificity(key, range.clone()) {
                if specificity.is_none() || s > specificity.unwrap() {
                    specificity = Some(s);
                    result = Some(v);
                }
            }
        }

        result.unwrap_or_default()
    }

    fn layout_text(&mut self, text: &TextSection<'_>) -> Option<Glyphs> {
        self.fonts_mut().layout_glyphs(text)
    }

    /// Measures the given text.
    fn measure_text(&mut self, text: &TextSection<'_>) -> Vec2 {
        self.fonts_mut().measure_text(text).unwrap_or_default()
    }

    /// Tries to downcast the `renderer` to the given type.
    fn downcast_renderer<T: Renderer>(&self) -> Option<&T> {
        self.renderer().downcast_ref()
    }

    /// Loads an image from the given `source` and returns a handle to it.
    fn load_image(&mut self, source: impl Into<ImageSource>) -> ImageHandle {
        let source = source.into();
        if let Some(handle) = self.image_cache().get(&source) {
            return handle;
        }

        let data = source.clone().load();
        let image = self.renderer().create_image(&data);
        self.image_cache_mut().insert(source, image.clone());
        image
    }

    /// Returns `true` if the element is active.
    fn active(&self) -> bool {
        self.node().active
    }

    /// Returns `true` if the element is hovered.
    fn hovered(&self) -> bool {
        self.node().hovered
    }

    /// Returns `true` if the element is focused.
    fn focused(&self) -> bool {
        self.node().focused
    }

    /// Focuses the element, this will also request a redraw.
    fn focus(&mut self) {
        if self.focused() {
            return;
        }

        self.node_mut().focused = true;
        self.request_redraw();
    }

    /// Unfocuses the element, this will also request a redraw.
    fn unfocus(&mut self) {
        if !self.focused() {
            return;
        }

        self.node_mut().focused = false;
        self.request_redraw();
    }

    /// Hovers the element, this will also request a redraw.
    fn hover(&mut self) {
        if self.hovered() {
            return;
        }

        self.node_mut().hovered = true;
        self.request_redraw();
    }

    /// Unhovers the element, this will also request a redraw.
    fn unhover(&mut self) {
        if !self.hovered() {
            return;
        }

        self.node_mut().hovered = false;
        self.request_redraw();
    }

    /// Activates the element, this will also request a redraw.
    fn activate(&mut self) {
        if self.active() {
            return;
        }

        self.node_mut().active = true;
        self.request_redraw();
    }

    /// Deactivates the element, this will also request a redraw.
    fn deactivate(&mut self) {
        if !self.active() {
            return;
        }

        self.node_mut().active = false;
        self.request_redraw();
    }

    /// Returns the local rect of the element.
    fn local_rect(&self) -> Rect {
        self.node().local_rect
    }

    /// Returns the global rect of the element.
    fn rect(&self) -> Rect {
        self.node().global_rect
    }

    /// Returns the margin of the element.
    fn margin(&self) -> Margin {
        self.node().margin
    }

    /// Returns the padding of the element.
    fn padding(&self) -> Padding {
        self.node().padding
    }

    /// Returns the size of the element.
    fn size(&self) -> Vec2 {
        self.node().local_rect.size()
    }

    /// Requests a redraw.
    ///
    /// This is a shortcut for `self.event_sink().send(RequestRedrawEvent)`.
    #[track_caller]
    fn request_redraw(&mut self) {
        tracing::trace!("request redraw");
        self.send_event(RequestRedrawEvent);
    }

    /// Requests a layout.
    ///
    /// This is a shortcut for `self.state_mut().needs_layout = true`.
    #[track_caller]
    fn request_layout(&mut self) {
        tracing::trace!("request layout");
        self.node_mut().needs_layout = true;
    }

    /// Sends an event to the event sink.
    fn send_event(&self, event: impl Any + Send + Sync) {
        self.event_sink().emit(event);
    }

    /// Returns the time in seconds since the last frame.
    fn delta_time(&self) -> f32 {
        self.node().delta_time()
    }
}

macro_rules! context {
    ($name:ident) => {
        impl<'a> Context for $name<'a> {
            fn stylesheet(&self) -> &Stylesheet {
                self.stylesheet
            }

            fn style_cache(&self) -> &StyleCache {
                self.style_cache
            }

            fn style_cache_mut(&mut self) -> &mut StyleCache {
                self.style_cache
            }

            fn stylesheet_and_cache_mut(&mut self) -> (&Stylesheet, &mut StyleCache) {
                (self.stylesheet, self.style_cache)
            }

            fn query_style_attribute(&mut self, key: &str) -> Option<(StyleAttribute, StyleSpec)> {
                let cache = &mut self.style_cache;
                let tree = &self.style_tree;
                self.stylesheet.query_cached(cache, None, tree, key)
            }

            fn node(&self) -> &NodeState {
                self.node
            }

            fn node_mut(&mut self) -> &mut NodeState {
                self.node
            }

            fn renderer(&self) -> &dyn Renderer {
                self.renderer
            }

            fn window(&self) -> &Window {
                self.window
            }

            fn window_mut(&mut self) -> &mut Window {
                self.window
            }

            fn fonts(&self) -> &Fonts {
                self.fonts
            }

            fn fonts_mut(&mut self) -> &mut Fonts {
                self.fonts
            }

            fn style_tree(&self) -> &StyleTree {
                self.style_tree
            }

            fn style_tree_mut(&mut self) -> &mut StyleTree {
                self.style_tree
            }

            fn event_sink(&self) -> &EventSink {
                &self.event_sink
            }

            fn image_cache(&self) -> &ImageCache {
                &self.image_cache
            }

            fn image_cache_mut(&mut self) -> &mut ImageCache {
                &mut self.image_cache
            }
        }
    };
}

context!(EventContext);
context!(LayoutContext);
context!(DrawContext);
