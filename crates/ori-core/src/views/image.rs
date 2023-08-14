use ori_graphics::{math::Vec2, ImageFilter, ImageHandle, ImageSource, Mesh};
use ori_reactive::Event;

use crate::{
    AvailableSpace, Context, DrawContext, EventContext, Key, LayoutContext, Length, Size,
    StateView, Style,
};

/// An image view.
#[derive(Clone, Debug, Default)]
pub struct Image {
    /// The source of the image to display.
    pub image: ImageSource,
    /// The size of the image.
    pub size: Size,
    /// The filter of the image.
    pub filter: Style<ImageFilter>,
}

impl Image {
    pub const FILTER: Key<ImageFilter> = Key::new("image.filter");

    /// Create a new image view.
    pub fn new(image: impl Into<ImageSource>) -> Self {
        Self {
            image: image.into(),
            size: Size::content(),
            filter: Style::new(Self::FILTER),
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
    pub fn min_width(mut self, min_width: impl Into<Length>) -> Self {
        self.size.min_width = min_width.into();
        self
    }

    /// Set the max width.
    pub fn max_width(mut self, max_width: impl Into<Length>) -> Self {
        self.size.max_width = max_width.into();
        self
    }

    /// Set the min height.
    pub fn min_height(mut self, min_height: impl Into<Length>) -> Self {
        self.size.min_height = min_height.into();
        self
    }

    /// Set the max height.
    pub fn max_height(mut self, max_height: impl Into<Length>) -> Self {
        self.size.max_height = max_height.into();
        self
    }

    /// Set the filter.
    pub fn filter(mut self, filter: impl Into<Style<ImageFilter>>) -> Self {
        self.filter = filter.into();
        self
    }
}

impl StateView for Image {
    type State = ImageHandle;

    fn build(&mut self, cx: &mut Context<'_>) -> Self::State {
        if let Some(handle) = cx.image_cache.get(&self.image) {
            return handle;
        }

        let mut data = self.image.clone().load();
        data.set_filter(self.filter.get(cx.theme));
        let handle = cx.renderer.create_image(&data);
        cx.image_cache.insert(self.image.clone(), handle.clone());
        handle
    }

    fn event(&mut self, _state: &mut Self::State, _cx: &mut EventContext<'_>, _event: &Event) {}

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        self.size.resolve(cx, state.size(), space)
    }

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        let mesh = Mesh::image(cx.rect(), state.clone());
        cx.draw(mesh);
    }
}
