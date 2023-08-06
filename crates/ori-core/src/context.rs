use std::{
    any::Any,
    mem,
    ops::{Deref, DerefMut, Range},
};

use glam::Vec2;
use ori_graphics::{
    Color, Fonts, Frame, Glyphs, ImageCache, ImageHandle, ImageSource, PrimitiveKind, Quad, Rect,
    Renderer, TextSection,
};
use ori_reactive::{EventSink, Signal};
use ori_style::{
    FromStyleAttribute, Length, StyleAttribute, StyleCache, StyleSpec, StyleTree, Stylesheet,
};

use crate::{AvailableSpace, Margin, NodeState, Padding, RequestRedrawEvent, Window};

/// A context for [`Element::event`](crate::Element::event).
#[allow(missing_docs)]
pub struct EventContext<'a> {
    pub context: Context<'a>,
}

impl<'a> Deref for EventContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> DerefMut for EventContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}

impl<'a> EventContext<'a> {
    pub fn offset(&mut self, offset: Vec2, f: impl FnOnce(&mut Self)) {
        self.offset += offset;
        f(self);
        self.offset -= offset;
    }
}

/// A context for [`Element::layout`](crate::Element::layout).
#[allow(missing_docs)]
pub struct LayoutContext<'a> {
    pub context: Context<'a>,
    pub space: AvailableSpace,
    pub parent_space: AvailableSpace,
}

impl<'a> Deref for LayoutContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> DerefMut for LayoutContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}

impl<'a> LayoutContext<'a> {
    /// Gets the available space, constrained by the element's style.
    pub fn style_constraints(&mut self, space: AvailableSpace) -> AvailableSpace {
        let size = self.query_style::<Length>("size");

        let width = self.query_style::<Length>("width");
        let height = self.query_style::<Length>("height");

        let min_width = self.query_style::<Length>("min-width");
        let max_width = self.query_style::<Length>("max-width");
        let min_height = self.query_style::<Length>("min-height");
        let max_height = self.query_style::<Length>("max-height");

        let min_width = StyleSpec::select(&[min_width, size, width], Length::ZERO);
        let max_width = StyleSpec::select(&[max_width, size, width], Length::INFINITY);
        let min_height = StyleSpec::select(&[min_height, size, height], Length::ZERO);
        let max_height = StyleSpec::select(&[max_height, size, height], Length::INFINITY);

        let parent = self.parent_space;
        let min_width = self.resolve_length(min_width, 0.0..parent.max.x);
        let max_width = self.resolve_length(max_width, 0.0..parent.max.x);
        let min_height = self.resolve_length(min_height, 0.0..parent.max.y);
        let max_height = self.resolve_length(max_height, 0.0..parent.max.y);

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
    offset: Option<Vec2>,
    clip: Option<Rect>,
}

impl<'a, 'b> DrawLayer<'a, 'b> {
    /// Set the z-index of the layer.
    pub fn z_index(mut self, depth: f32) -> Self {
        self.z_index = depth;
        self
    }

    /// Set the rectangle for the layer.
    pub fn offset(mut self, offset: Vec2) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the clipping rectangle for the layer.
    pub fn clip(mut self, clip: Rect) -> Self {
        self.clip = Some(clip);
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
            self.draw_context.context.cloned(|mut context| {
                context.offset += self.offset.unwrap_or(Vec2::ZERO);

                let mut child = DrawContext {
                    context,
                    parent_size: self.draw_context.parent_size,
                    frame,
                };

                f(&mut child);
            });
        });
    }
}

/// A context for [`Element::draw`](crate::Element::draw).
#[allow(missing_docs)]
pub struct DrawContext<'a> {
    pub context: Context<'a>,
    pub frame: &'a mut Frame,
    pub parent_size: Vec2,
}

impl<'a> Deref for DrawContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> DerefMut for DrawContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
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
            offset: None,
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
        let fonts = &mut self.context.fonts;
        let renderer = self.context.renderer;

        if let Some(mesh) = fonts.text_mesh(renderer, glyphs, rect) {
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

        let radius = format!("{}-radius", name);
        let radius = self.query_style::<Length>(&radius);

        let tl = self.query_style::<Length>(&tl);
        let tr = self.query_style::<Length>(&tr);
        let bl = self.query_style::<Length>(&bl);
        let br = self.query_style::<Length>(&br);

        let tl = StyleSpec::select(&[tl, radius], Length::default());
        let tr = StyleSpec::select(&[tr, radius], Length::default());
        let bl = StyleSpec::select(&[bl, radius], Length::default());
        let br = StyleSpec::select(&[br, radius], Length::default());

        let min_element = parent_size.min_element();

        [
            self.resolve_length(tl, 0.0..min_element),
            self.resolve_length(tr, 0.0..min_element),
            self.resolve_length(bl, 0.0..min_element),
            self.resolve_length(br, 0.0..min_element),
        ]
    }

    /// Gets the border width for the given element.
    pub fn style_border_width(&mut self, name: &str, parent_size: Vec2) -> [f32; 4] {
        let t = format!("{}-top-width", name);
        let r = format!("{}-right-width", name);
        let b = format!("{}-bottom-width", name);
        let l = format!("{}-left-width", name);

        let width = format!("{}-width", name);
        let width = self.query_style::<Length>(&width);

        let t = self.query_style::<Length>(&t);
        let r = self.query_style::<Length>(&r);
        let b = self.query_style::<Length>(&b);
        let l = self.query_style::<Length>(&l);

        let t = StyleSpec::select(&[t, width], Length::default());
        let r = StyleSpec::select(&[r, width], Length::default());
        let b = StyleSpec::select(&[b, width], Length::default());
        let l = StyleSpec::select(&[l, width], Length::default());

        [
            self.resolve_length(t, 0.0..parent_size.y),
            self.resolve_length(r, 0.0..parent_size.x),
            self.resolve_length(b, 0.0..parent_size.y),
            self.resolve_length(l, 0.0..parent_size.x),
        ]
    }

    /// Gets a styled quad with the given background and border names.
    pub fn style_quad_named(&mut self, background: &str, border: &str, parent_size: Vec2) -> Quad {
        let background_image = format!("{}-image", background);
        let background_color = format!("{}-color", background);
        let border_color = format!("{}-color", border);

        let background_image = self.get_style_image_group(&[&background_image, background]);

        let background_color = if background_image.is_some() {
            self.get_style(&background_color).unwrap_or(Color::WHITE)
        } else {
            self.style_group(&[&background_color, background])
        };

        let border_radius = self.style_border_radius(border, parent_size);
        let border_width = self.style_border_width(border, parent_size);
        let border_color = self.style(&border_color);

        Quad {
            rect: self.rect(),
            background_color,
            background_image,
            border_radius,
            border_width,
            border_color,
        }
    }

    /// Gets the styled background quad.
    ///
    /// This will use the following style attributes:
    /// - `background[-color]`: The background color of the quad.
    /// - `background[-image]`: The background image of the quad.
    /// - `border[-top-left,-top-right,bottom-right,bottom-left]-radius`: The border radi of the quad.
    /// - `border[-top,-right,-bottom,-left]-width`: The border width of the quad.
    /// - `border-color`: The border color of the quad.
    pub fn style_background(&mut self) -> Quad {
        self.style_quad_named("background", "border", self.parent_size)
    }

    /// Draws the quad at the current layout rect, see [`Context::style_background`].
    pub fn draw_background(&mut self) {
        let quad = self.style_background();
        self.draw(quad);
    }

    /// Draw a [`Primitive`].
    pub fn draw(&mut self, primitive: impl Into<PrimitiveKind>) {
        self.frame.draw(primitive);
    }
}

pub struct Context<'a> {
    pub node: &'a mut NodeState,
    pub renderer: &'a dyn Renderer,
    pub window: Signal<Window>,
    pub fonts: &'a mut Fonts,
    pub stylesheet: &'a Stylesheet,
    pub style_tree: &'a mut StyleTree,
    pub style_cache: &'a mut StyleCache,
    pub event_sink: &'a EventSink,
    pub image_cache: &'a mut ImageCache,
    pub offset: Vec2,
    window_size: Vec2,
    window_scale: f32,
}

impl<'a> Context<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        node: &'a mut NodeState,
        renderer: &'a dyn Renderer,
        window: Signal<Window>,
        fonts: &'a mut Fonts,
        stylesheet: &'a Stylesheet,
        style_tree: &'a mut StyleTree,
        style_cache: &'a mut StyleCache,
        event_sink: &'a EventSink,
        image_cache: &'a mut ImageCache,
    ) -> Self {
        let offset = node.rect.top_left();

        Self {
            node,
            renderer,
            window,
            fonts,
            stylesheet,
            style_tree,
            style_cache,
            event_sink,
            image_cache,
            offset,
            window_size: window.get().size.as_vec2(),
            window_scale: window.get().scale,
        }
    }

    pub(crate) fn child<T>(&mut self, node: &mut NodeState, f: impl FnOnce(Context<'_>) -> T) -> T {
        node.propagate_up(self.node);

        if node.needs_layout {
            self.request_redraw();
        }

        node.update_style_tags();
        self.style_tree.push(node.style.clone());

        let offset = self.rect().top_left();
        let context = Context {
            node,
            renderer: self.renderer,
            window: self.window,
            fonts: self.fonts,
            stylesheet: self.stylesheet,
            style_tree: self.style_tree,
            style_cache: self.style_cache,
            event_sink: self.event_sink,
            image_cache: self.image_cache,
            offset,
            window_size: self.window_size,
            window_scale: self.window_scale,
        };

        let result = f(context);
        self.style_tree.pop();

        self.node.propagate_down(node);

        result
    }

    pub(crate) fn cloned<T>(&mut self, f: impl FnOnce(Context<'_>) -> T) -> T {
        let context = Context {
            node: self.node,
            renderer: self.renderer,
            window: self.window,
            fonts: self.fonts,
            stylesheet: self.stylesheet,
            style_tree: self.style_tree,
            style_cache: self.style_cache,
            event_sink: self.event_sink,
            image_cache: self.image_cache,
            offset: self.offset,
            window_size: self.window_size,
            window_scale: self.window_scale,
        };

        f(context)
    }

    #[inline(always)]
    pub fn window_size(&self) -> Vec2 {
        self.window_size
    }

    #[inline(always)]
    pub fn window_scale(&self) -> f32 {
        self.window_scale
    }

    #[inline(always)]
    pub fn resolve_length(&self, length: Length, range: Range<f32>) -> f32 {
        length.pixels(
            range,
            self.window_scale,
            self.window_size.x,
            self.window_size.y,
        )
    }

    #[inline(always)]
    pub fn query_style_attribute(&mut self, key: &str) -> Option<(StyleAttribute, StyleSpec)> {
        let cache = &mut self.style_cache;
        let tree = &self.style_tree;
        let query = self.stylesheet.query_cached(cache, tree, key)?;

        if query.inherited && !self.node.is_inheriting(query.attribute.key()) {
            self.node.inheriting.push(query.attribute.clone());
        }

        Some((query.attribute, query.specificity))
    }

    /// Gets the [`StyleAttribute`] for the given `key`.
    #[inline(always)]
    pub fn get_style_attribute(&mut self, key: &str) -> Option<StyleAttribute> {
        self.query_style_attribute(key)
            .map(|(attribute, _)| attribute)
    }

    /// Gets the value of a style attribute for the given `key`.
    #[inline(always)]
    pub fn query_style<T: FromStyleAttribute + 'static>(
        &mut self,
        key: &str,
    ) -> Option<(T, StyleSpec)> {
        let (attribute, specificity) = self.query_style_attribute(key)?;
        let value = T::from_attribute(attribute.value().clone())?;
        let transition = attribute.transition();

        Some((self.node.transition(key, value, transition), specificity))
    }

    /// Gets the value of a style attribute for the given `key`.
    ///
    /// This will also transition the value if the attribute has a transition.
    #[inline(always)]
    pub fn get_style<T: FromStyleAttribute + 'static>(&mut self, key: &str) -> Option<T> {
        self.query_style(key).map(|(value, _)| value)
    }

    /// Gets the value of a style attribute for the given `key`, if there is no value, returns `T::default()`.
    ///
    /// This will also transition the value if the attribute has a transition.
    #[track_caller]
    #[inline(always)]
    pub fn style<T: FromStyleAttribute + Default + 'static>(&mut self, key: &str) -> T {
        self.get_style(key).unwrap_or_default()
    }

    /// Takes a `primary_key` and a `secondary_key` and returns the value of the attribute with the highest specificity.
    /// If both attributes have the same specificity, the `primary_key` will be used.
    ///
    /// This will also transition the value if the attribute has a transition.
    #[inline(always)]
    pub fn style_group<T: FromStyleAttribute + Default + 'static>(&mut self, keys: &[&str]) -> T {
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
    #[inline(always)]
    pub fn get_style_length(&mut self, key: &str, range: Range<f32>) -> Option<f32> {
        let attribute = self.get_style_attribute(key)?;
        let value = Length::from_attribute(attribute.value().clone())?;
        let transition = attribute.transition();

        let pixels = self.resolve_length(value, range);
        Some((self.node).transition(key, pixels, transition))
    }

    /// Gets the value of a style attribute in pixels and [`StyleSpec`] for the given `key`.
    #[inline(always)]
    pub fn get_style_length_specificity(
        &mut self,
        key: &str,
        range: Range<f32>,
    ) -> Option<(f32, StyleSpec)> {
        let (attribute, specificity) = self.query_style_attribute(key)?;
        let value = Length::from_attribute(attribute.value().clone())?;
        let transition = attribute.transition();

        let pixels = self.resolve_length(value, range);
        Some(((self.node).transition(key, pixels, transition), specificity))
    }

    /// Gets the value of a style attribute in pixels for the given `key`, if there is no value, returns `0.0`.
    /// `range` is the range from 0% to 100% of the desired value.
    ///
    /// This will also transition the value if the attribute has a transition.
    #[track_caller]
    #[inline(always)]
    pub fn style_length(&mut self, key: &str, range: Range<f32>) -> f32 {
        self.get_style_length(key, range).unwrap_or_default()
    }

    /// Takes a `primary_key` and a `secondary_key` and returns the value of the attribute with the highest specificity in pixels.
    /// If both attributes have the same specificity, the `primary_key` will be used.
    /// `range` is the range from 0% to 100% of the desired value.
    ///
    /// This will also transition the value if the attribute has a transition.
    #[inline(always)]
    pub fn style_length_group(&mut self, keys: &[&str], range: Range<f32>) -> f32 {
        let mut specificity = None;
        let mut result = None;

        for key in keys {
            if let Some((v, s)) = self.get_style_length_specificity(key, range.clone()) {
                if specificity.is_none() || s > specificity.unwrap() {
                    specificity = Some(s);
                    result = Some(v);
                }
            }
        }

        result.unwrap_or_default()
    }

    /// Gets an optional image handle for the given `key`.
    #[inline(always)]
    pub fn get_style_image(&mut self, key: &str) -> Option<ImageHandle> {
        let image = self.style::<Option<ImageSource>>(key);
        image.map(|image| self.load_image(image))
    }

    /// Gets an optional image handle for the given `keys`, prioritizing in order.
    #[inline(always)]
    pub fn get_style_image_group(&mut self, keys: &[&str]) -> Option<ImageHandle> {
        let image = self.style_group::<Option<ImageSource>>(keys);
        image.map(|image| self.load_image(image))
    }

    /// Layout a section of text.
    pub fn layout_text(&mut self, text: &TextSection<'_>) -> Option<Glyphs> {
        self.fonts.layout_glyphs(text)
    }

    /// Measures the given text.
    pub fn measure_text(&mut self, text: &TextSection<'_>) -> Vec2 {
        self.fonts.measure_text(text).unwrap_or_default()
    }

    /// Tries to downcast the `renderer` to the given type.
    pub fn downcast_renderer<T: Renderer>(&self) -> Option<&T> {
        self.renderer.downcast_ref()
    }

    /// Loads an image from the given `source` and returns a handle to it.
    pub fn load_image(&mut self, source: impl Into<ImageSource>) -> ImageHandle {
        let source = source.into();
        if let Some(handle) = self.image_cache.get(&source) {
            return handle;
        }

        let data = source.clone().load();
        let image = self.renderer.create_image(&data);
        self.image_cache.insert(source, image.clone());
        image
    }

    /// Returns `true` if the element is active.
    pub fn active(&self) -> bool {
        self.node.active
    }

    /// Returns `true` if the element is hovered.
    pub fn hovered(&self) -> bool {
        self.node.hovered
    }

    /// Returns `true` if the element is focused.
    pub fn focused(&self) -> bool {
        self.node.focused
    }

    /// Focuses the element, this will also request a redraw.
    pub fn focus(&mut self) {
        if self.focused() {
            return;
        }

        self.node.focused = true;
        self.request_redraw();
    }

    /// Unfocuses the element, this will also request a redraw.
    pub fn unfocus(&mut self) {
        if !self.focused() {
            return;
        }

        self.node.focused = false;
        self.request_redraw();
    }

    /// Hovers the element, this will also request a redraw.
    pub fn hover(&mut self) {
        if self.hovered() {
            return;
        }

        self.node.hovered = true;
        self.request_redraw();
    }

    /// Unhovers the element, this will also request a redraw.
    pub fn unhover(&mut self) {
        if !self.hovered() {
            return;
        }

        self.node.hovered = false;
        self.request_redraw();
    }

    /// Activates the element, this will also request a redraw.
    pub fn activate(&mut self) {
        if self.active() {
            return;
        }

        self.node.active = true;
        self.request_redraw();
    }

    /// Deactivates the element, this will also request a redraw.
    pub fn deactivate(&mut self) {
        if !self.active() {
            return;
        }

        self.node.active = false;
        self.request_redraw();
    }

    /// Returns the local rect of the element.
    pub fn local_rect(&self) -> Rect {
        self.node.rect
    }

    /// Returns the global rect of the element.
    pub fn rect(&self) -> Rect {
        self.node.rect.translate(self.offset)
    }

    /// Returns the margin of the element.
    pub fn margin(&self) -> Margin {
        self.node.margin
    }

    /// Returns the padding of the element.
    pub fn padding(&self) -> Padding {
        self.node.padding
    }

    /// Returns the size of the element.
    pub fn size(&self) -> Vec2 {
        self.node.rect.size()
    }

    /// Requests a redraw.
    ///
    /// This is a shortcut for `self.event_sink().send(RequestRedrawEvent)`.
    #[track_caller]
    pub fn request_redraw(&mut self) {
        tracing::trace!("request redraw");
        self.send_event(RequestRedrawEvent);
    }

    /// Requests a layout.
    ///
    /// This is a shortcut for `self.state_mut().needs_layout = true`.
    #[track_caller]
    pub fn request_layout(&mut self) {
        tracing::trace!("request layout");
        self.node.needs_layout = true;
    }

    /// Sends an event to the event sink.
    pub fn send_event(&self, event: impl Any + Send + Sync) {
        self.event_sink.emit(event);
    }

    /// Returns the time in seconds since the last frame.
    pub fn delta_time(&self) -> f32 {
        self.node.delta_time()
    }
}
