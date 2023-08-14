use ori_graphics::{math::Vec2, Color, ImageHandle, ImageSource, Quad};
use ori_reactive::Event;

use crate::{
    AvailableSpace, BorderRadius, BorderWidth, Context, DrawContext, EventContext, Key,
    LayoutContext, Node, StateView, Style, Styled,
};

/// A view that decorates its content with a background quad.
#[derive(Debug)]
pub struct Decorate {
    /// The content to decorate.
    pub content: Node,
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

impl Default for Decorate {
    fn default() -> Self {
        Self {
            content: Node::empty(),
            background_color: Style::new(Self::BACKGROUND_COLOR),
            background_image: None,
            border_width: Style::new(Self::BORDER_WIDTH),
            border_radius: Style::new(Self::BORDER_RADIUS),
            border_color: Style::new(Self::BORDER_COLOR),
        }
    }
}

impl Decorate {
    pub const BACKGROUND_COLOR: Key<Color> = Key::new("decorate.background-color");
    pub const BORDER_WIDTH: Key<BorderWidth> = Key::new("decorate.border-width");
    pub const BORDER_RADIUS: Key<BorderRadius> = Key::new("decorate.border-radius");
    pub const BORDER_COLOR: Key<Color> = Key::new("decorate.border-color");

    /// Create a new decorate view.
    pub fn new(content: impl Into<Node>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
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
}

impl StateView for Decorate {
    type State = Option<ImageHandle>;

    fn build(&mut self, cx: &mut Context<'_>) -> Self::State {
        let source = self.background_image.as_ref()?;

        if let Some(handle) = cx.image_cache.get(source) {
            return Some(handle);
        }

        let data = source.clone().load();
        let handle = cx.renderer.create_image(&data);
        cx.image_cache.insert(source.clone(), handle.clone());
        Some(handle)
    }

    fn event(&mut self, _state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        self.content.event(cx, event);
    }

    fn layout(
        &mut self,
        _state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        self.content.layout(cx, space)
    }

    fn draw(&mut self, handle: &mut Self::State, cx: &mut DrawContext<'_>) {
        cx.draw(Quad {
            rect: cx.rect(),
            background_color: self.background_color.get(cx.theme),
            background_image: handle.clone(),
            border_radius: self.border_radius.get(cx.theme).get(cx),
            border_width: self.border_width.get(cx.theme).get(cx),
            border_color: self.border_color.get(cx.theme),
        });

        self.content.draw(cx);
    }
}
