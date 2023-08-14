use ori_graphics::{math::Vec2, Affine, Color, ImageHandle, ImageSource, Quad};
use ori_reactive::Event;

use crate::{
    Alignment, AvailableSpace, BorderRadius, BorderWidth, Context, DrawContext, EventContext, Key,
    LayoutContext, Length, Node, Padding, Size, StateView, Style, Styled, Unit,
};

/// A container view.
#[derive(Debug)]
pub struct Container {
    /// The content to contain.
    pub content: Node,
    /// The size of the container.
    pub size: Size,
    /// The padding.
    pub padding: Padding,
    /// The transform.
    pub transform: Affine,
    /// The alignment.
    pub alignment: Alignment,
    /// The background color.
    pub background_color: Style<Color>,
    /// The background image.
    pub background_image: Option<ImageSource>,
    /// The border width.
    pub border_width: Style<BorderWidth>,
    /// The border radius.
    pub border_radius: Style<BorderRadius>,
    /// The border color.
    pub border_color: Style<Color>,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            content: Node::empty(),
            size: Size::content(),
            padding: Padding::all(Unit::ZERO),
            transform: Affine::IDENTITY,
            alignment: Alignment::center(),
            background_color: Style::new(Self::BACKGROUND_COLOR),
            background_image: None,
            border_width: Style::new(Self::BORDER_WIDTH),
            border_radius: Style::new(Self::BORDER_RADIUS),
            border_color: Style::new(Self::BORDER_COLOR),
        }
    }
}

impl Container {
    pub const BACKGROUND_COLOR: Key<Color> = Key::new("decorate.background-color");
    pub const BORDER_WIDTH: Key<BorderWidth> = Key::new("decorate.border-width");
    pub const BORDER_RADIUS: Key<BorderRadius> = Key::new("decorate.border-radius");
    pub const BORDER_COLOR: Key<Color> = Key::new("decorate.border-color");

    /// Create a new container with `content`.
    pub fn new(content: impl Into<Node>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Set the size.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Set the width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.set_width(width);
        self
    }

    /// Set the height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.set_height(height);
        self
    }

    /// Set the min width.
    pub fn min_width(mut self, width: impl Into<Length>) -> Self {
        self.size.min_width = width.into();
        self
    }

    /// Set the max width.
    pub fn max_width(mut self, width: impl Into<Length>) -> Self {
        self.size.max_width = width.into();
        self
    }

    /// Set the min height.
    pub fn min_height(mut self, height: impl Into<Length>) -> Self {
        self.size.min_height = height.into();
        self
    }

    /// Set the max height.
    pub fn max_height(mut self, height: impl Into<Length>) -> Self {
        self.size.max_height = height.into();
        self
    }

    /// Set the padding.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Set the transform.
    pub fn transform(mut self, transform: impl Into<Affine>) -> Self {
        self.transform = transform.into();
        self
    }

    /// Set the alignment.
    pub fn alignment(mut self, alignment: impl Into<Alignment>) -> Self {
        self.alignment = alignment.into();
        self
    }

    /// Set the background color.
    pub fn background_color(mut self, color: impl Styled<Color>) -> Self {
        self.background_color = color.style();
        self
    }

    /// Set the background image.
    pub fn background_image(mut self, image: impl Into<ImageSource>) -> Self {
        self.background_image = Some(image.into());
        self
    }

    /// Set the border width.
    pub fn border_width(mut self, width: impl Styled<BorderWidth>) -> Self {
        self.border_width = width.style();
        self
    }

    /// Set the border radius.
    pub fn border_radius(mut self, radius: impl Styled<BorderRadius>) -> Self {
        self.border_radius = radius.style();
        self
    }

    /// Set the border color.
    pub fn border_color(mut self, color: impl Styled<Color>) -> Self {
        self.border_color = color.style();
        self
    }

    fn image_handle(&self, cx: &mut Context<'_>) -> Option<ImageHandle> {
        let source = self.background_image.as_ref()?;

        if let Some(handle) = cx.image_cache.get(source) {
            return Some(handle);
        }

        let data = source.clone().load();
        let handle = cx.renderer.create_image(&data);
        cx.image_cache.insert(source.clone(), handle.clone());
        Some(handle)
    }
}

#[doc(hidden)]
#[derive(Default)]
pub struct ContainerState {
    content_size: Vec2,
    background_image: Option<ImageHandle>,
}

impl StateView for Container {
    type State = ContainerState;

    fn build(&mut self, cx: &mut Context<'_>) -> Self::State {
        ContainerState {
            content_size: Vec2::ZERO,
            background_image: self.image_handle(cx),
        }
    }

    fn event(&mut self, state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        let align = self.alignment.align(state.content_size, cx.size());
        let transform = Affine::translation(align) * self.transform;

        cx.with_transform(transform, |cx| {
            self.content.event_padded(cx, event, self.padding);
        });
    }

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        let padded_space = space.shrink(self.padding.size(cx));
        let content_space = self.size.content_space(cx, padded_space);
        state.content_size = self.content.layout_padded(cx, content_space, self.padding);
        self.size.resolve(cx, state.content_size, space)
    }

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        let align = self.alignment.align(state.content_size, cx.size());
        let transform = Affine::translation(align) * self.transform;

        cx.with_transform(transform, |cx| {
            cx.draw(Quad {
                rect: cx.rect(),
                background_color: self.background_color.get(cx.theme),
                background_image: state.background_image.clone(),
                border_radius: self.border_radius.get(cx.theme).get(cx),
                border_width: self.border_width.get(cx.theme).get(cx),
                border_color: self.border_color.get(cx.theme),
            });

            self.content.draw_padded(cx, self.padding);
        });
    }
}
