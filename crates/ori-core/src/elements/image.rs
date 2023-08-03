use glam::Vec2;
use ori_graphics::{ImageFilter, ImageHandle, ImageSource, Mesh};
use ori_macro::Build;
use ori_style::Style;

use crate::{AvailableSpace, Context, DrawContext, Element, LayoutContext};

/// A element that displays an image.
#[derive(Clone, Default, Debug, Build)]
pub struct Image {
    /// The filter to apply to the image.
    #[prop]
    pub filter: ImageFilter,
    /// The source of the image.
    #[prop]
    pub src: ImageSource,
}

impl Image {
    /// Creates a new image.
    pub fn new() -> Self {
        Self::default()
    }
}

/// The state of an image element.
#[derive(Clone, Debug, Default)]
pub struct ImageState {
    src: ImageSource,
    filter: ImageFilter,
    handle: Option<ImageHandle>,
}

impl ImageState {
    /// Updates the image handle if the source has changed.
    pub fn update(
        &mut self,
        context: &mut impl Context,
        src: &ImageSource,
        filter: ImageFilter,
    ) -> &ImageHandle {
        if self.src != *src || self.filter != filter || self.handle.is_none() {
            self.src = src.clone();
            self.filter = filter;

            let mut data = src.clone().load();
            data.set_filter(filter);

            self.handle = Some(context.load_image(data));
        }

        self.handle.as_ref().unwrap()
    }
}

impl Element for Image {
    type State = ImageState;

    fn build(&self) -> Self::State {
        Default::default()
    }

    fn style(&self) -> Style {
        Style::new("image")
    }

    fn layout(
        &self,
        state: &mut Self::State,
        cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        let handle = state.update(cx, &self.src, self.filter);
        let size = handle.size();

        let min = space.min / size;
        let max = space.max / size;

        let min = min.max_element();
        let max = max.min_element();

        if min > max {
            return space.constrain(size);
        }

        size * max.min(1.0) * min.max(1.0)
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext) {
        let handle = state.update(cx, &self.src, self.filter);
        let mesh = Mesh::image(cx.rect(), handle.clone());
        cx.draw(mesh);
    }
}
