use std::sync::Arc;

use ori_core::{prelude::Build, AvailableSpace, DrawContext, Element, EventContext, LayoutContext};
use ori_graphics::{prelude::Vec2, ImageFilter, ImageHandle, Mesh, Rect};
use ori_reactive::{Emitter, Event};
use ori_style::Style;
use wgpu::{Device, FilterMode, Queue, TextureUsages};

use crate::{WgpuImage, WgpuRenderer};

/// A surface for rendering a [`WgpuImage`].
#[derive(Debug)]
pub struct WgpuSurface {
    /// The wgpu device for the surface.
    pub device: Arc<Device>,
    /// The wgpu queue for the surface.
    pub queue: Arc<Queue>,
    /// The image for the surface.
    pub image: Arc<WgpuImage>,
    /// The rect of the surface.
    pub rect: Rect,
}

impl WgpuSurface {
    fn new(renderer: &WgpuRenderer, width: u32, height: u32, usages: TextureUsages) -> Self {
        let image = WgpuRenderer::create_image(
            renderer.device(),
            renderer.queue(),
            &renderer.image_bind_group_layout,
            width,
            height,
            &[],
            FilterMode::Linear,
            TextureUsages::RENDER_ATTACHMENT | usages,
        );

        Self {
            device: renderer.device_arc(),
            queue: renderer.queue_arc(),
            image: Arc::new(image),
            // this will be updated in the draw function
            rect: Rect::ZERO,
        }
    }
}

/// A element that displays a [`WgpuImage`].
///
/// Use [`WgpuCanvas::on_render`] to render to the image.
#[derive(Clone, Debug, Build)]
pub struct WgpuCanvas {
    /// The texture usages for the canvas.
    #[prop]
    pub texture_usages: TextureUsages,
    /// A event that is emitted when a event is received.
    #[event]
    pub on_event: Emitter<(Rect, Event)>,
    /// A event that is emitted when the canvas needs to be rendered.
    #[event]
    pub on_render: Emitter<WgpuSurface>,
}

impl Default for WgpuCanvas {
    fn default() -> Self {
        Self {
            texture_usages: TextureUsages::empty(),
            on_event: Default::default(),
            on_render: Default::default(),
        }
    }
}

impl Element for WgpuCanvas {
    type State = Option<WgpuSurface>;

    fn build(&self) -> Self::State {
        None
    }

    fn style(&self) -> Style {
        Style::new("canvas")
    }

    fn event(&self, _state: &mut Self::State, cx: &mut EventContext, event: &Event) {
        self.on_event.emit(&(cx.global_rect(), event.clone()));
    }

    fn layout(
        &self,
        _state: &mut Self::State,
        _cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        space.max
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext) {
        let rect = cx.global_rect();
        let width = cx.size().x as u32;
        let height = cx.size().y as u32;

        let surface = state.get_or_insert_with(|| {
            let surface = WgpuSurface::new(
                (cx.downcast_renderer()).expect("wgpu renderer isn't available"),
                width,
                height,
                self.texture_usages,
            );
            surface
        });

        surface.rect = rect;
        self.on_render.emit(surface);

        let image = surface.image.clone();
        let image = ImageHandle::from_arc(image, width, height, ImageFilter::Linear);
        cx.draw(Mesh::image(rect, image));
    }
}
