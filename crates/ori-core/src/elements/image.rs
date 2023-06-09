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
        let min_width = cx.style_range_group(&["min-width", "width"], space.x_axis());
        let max_width = cx.style_range_group(&["max-width", "width"], space.x_axis());

        let min_height = cx.style_range_group(&["min-width", "height"], space.y_axis());
        let max_height = cx.style_range_group(&["min-height", "height"], space.y_axis());

        let min_size = space.constrain(Vec2::new(min_width, min_height));
        let max_size = space.constrain(Vec2::new(max_width, max_height));

        let handle = state.update(cx, &self.src, self.filter);

        // try to fit the image in the min/max size
        // while maintaining the aspect ratio
        let mut size = handle.size();
        let aspect = size.x / size.y;

        if size.x > max_size.x {
            size.x = max_size.x;
            size.y = size.x / aspect;
        }

        if size.y > max_size.y {
            size.y = max_size.y;
            size.x = size.y * aspect;
        }

        if size.x < min_size.x {
            size.x = min_size.x;
            size.y = size.x / aspect;
        }

        if size.y < min_size.y {
            size.y = min_size.y;
            size.x = size.y * aspect;
        }

        size
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext) {
        let handle = state.update(cx, &self.src, self.filter);
        let mesh = Mesh::image(cx.rect(), handle.clone());
        cx.draw(mesh);
    }
}
