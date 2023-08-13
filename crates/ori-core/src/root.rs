use std::time::Duration;

use ori_graphics::{math::Vec2, Affine, Fonts, Frame, ImageCache, Renderer};
use ori_reactive::{Event, EventSink};

use crate::{
    AvailableSpace, Context, DrawContext, EventContext, LayoutContext, Node, Theme, Tree, Window,
};

impl Node {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn event_root(
        &mut self,
        fonts: &mut Fonts,
        renderer: &dyn Renderer,
        image_cache: &mut ImageCache,
        theme: &Theme,
        window: &Window,
        delta_time: Duration,
        event_sink: &EventSink,
        tree: &mut Tree,
        event: &Event,
    ) {
        let context = Context::new(
            fonts,
            renderer,
            image_cache,
            theme,
            window,
            delta_time,
            event_sink,
            tree,
        );
        let mut cx = EventContext::new(context, Affine::IDENTITY);
        self.event(&mut cx, event);
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn layout_root(
        &mut self,
        fonts: &mut Fonts,
        renderer: &dyn Renderer,
        image_cache: &mut ImageCache,
        theme: &Theme,
        window: &Window,
        delta_time: Duration,
        event_sink: &EventSink,
        tree: &mut Tree,
        space: AvailableSpace,
    ) -> Vec2 {
        let context = Context::new(
            fonts,
            renderer,
            image_cache,
            theme,
            window,
            delta_time,
            event_sink,
            tree,
        );
        let mut cx = LayoutContext::new(context);
        self.layout(&mut cx, space)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw_root(
        &mut self,
        fonts: &mut Fonts,
        renderer: &dyn Renderer,
        image_cache: &mut ImageCache,
        theme: &Theme,
        window: &Window,
        delta_time: Duration,
        event_sink: &EventSink,
        tree: &mut Tree,
        frame: &mut Frame,
    ) {
        let context = Context::new(
            fonts,
            renderer,
            image_cache,
            theme,
            window,
            delta_time,
            event_sink,
            tree,
        );
        let mut cx = DrawContext::new(context, frame);
        self.draw(&mut cx);
    }
}
