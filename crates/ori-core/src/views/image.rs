use ori_graphics::{math::Vec2, ImageFilter, ImageHandle, ImageSource, Mesh};
use ori_reactive::Event;

use crate::{
    AvailableSpace, Context, DrawContext, EventContext, Key, LayoutContext, Size, StateView, Style,
};

#[derive(Clone, Debug, Default)]
pub struct Image {
    pub image: ImageSource,
    pub size: Size,
    pub filter: Style<ImageFilter>,
}

impl Image {
    pub const FILTER: Key<ImageFilter> = Key::new("image.filter");

    pub fn new(image: impl Into<ImageSource>) -> Self {
        Self {
            image: image.into(),
            size: Size::content(),
            filter: Style::new(Self::FILTER),
        }
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

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
        self.size.get(cx, state.size(), space)
    }

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        let mesh = Mesh::image(cx.rect(), state.clone());
        cx.draw(mesh);
    }
}
