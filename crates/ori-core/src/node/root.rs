use glam::Vec2;
use ori_graphics::{Fonts, Frame, ImageCache, Rect, Renderer};
use ori_reactive::{Event, EventSink};
use ori_style::{StyleCache, StyleTree, Stylesheet};

use crate::{
    AvailableSpace, DebugEvent, DrawContext, EventContext, LayoutContext, Margin, Node, Padding,
    PointerEvent, RequestRedrawEvent, Window, WindowResizedEvent,
};

impl Node {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn event_root_inner(
        &self,
        stylesheet: &Stylesheet,
        style_cache: &mut StyleCache,
        renderer: &dyn Renderer,
        window: &mut Window,
        fonts: &mut Fonts,
        event_sink: &EventSink,
        event: &Event,
        image_cache: &mut ImageCache,
    ) {
        let element_state = &mut self.node_state();
        element_state.style = self.element().style();

        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if Self::handle_pointer_event(element_state, pointer_event, event.is_handled()) {
                event_sink.emit(RequestRedrawEvent);
            }
        }

        if event.is::<WindowResizedEvent>() {
            element_state.needs_layout = true;
        }

        let mut style_tree = StyleTree::new(element_state.style.clone());
        let mut cx = EventContext {
            node: element_state,
            renderer,
            window,
            fonts,
            stylesheet,
            style_tree: &mut style_tree,
            event_sink,
            style_cache,
            image_cache,
        };

        if let Some(event) = event.get::<DebugEvent>() {
            event.set_element(&mut cx, self);
        }

        (self.element()).event(self.element_state().as_mut(), &mut cx, event);
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn layout_root_inner(
        &self,
        stylesheet: &Stylesheet,
        style_cache: &mut StyleCache,
        renderer: &dyn Renderer,
        window: &mut Window,
        fonts: &mut Fonts,
        event_sink: &EventSink,
        image_cache: &mut ImageCache,
    ) -> Vec2 {
        let element_state = &mut self.node_state();
        element_state.style = self.element().style();
        element_state.needs_layout = false;

        let space = AvailableSpace::new(Vec2::ZERO, window.size.as_vec2());

        let mut style_tree = StyleTree::new(element_state.style.clone());
        let mut cx = LayoutContext {
            node: element_state,
            renderer,
            window,
            fonts,
            stylesheet,
            style_tree: &mut style_tree,
            event_sink,
            style_cache,
            image_cache,
            parent_space: space,
            space,
        };

        cx.node.margin = Margin::from_style(&mut cx, space);
        cx.node.padding = Padding::from_style(&mut cx, space);

        let space = cx.style_constraints(space);
        cx.space = space;

        let size = (self.element()).layout(self.element_state().as_mut(), &mut cx, space);

        element_state.available_space = space;
        element_state.local_rect = Rect::min_size(element_state.local_rect.min, size);
        element_state.global_rect = Rect::min_size(element_state.global_rect.min, size);

        size
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw_root_inner(
        &self,
        stylesheet: &Stylesheet,
        style_cache: &mut StyleCache,
        frame: &mut Frame,
        renderer: &dyn Renderer,
        window: &mut Window,
        fonts: &mut Fonts,
        event_sink: &EventSink,
        image_cache: &mut ImageCache,
    ) {
        let element_state = &mut self.node_state();
        element_state.style = self.element().style();

        let parent_size = window.size.as_vec2();
        let mut style_tree = StyleTree::new(element_state.style.clone());
        let mut cx = DrawContext {
            node: element_state,
            frame,
            renderer,
            window,
            fonts,
            parent_size,
            stylesheet,
            style_tree: &mut style_tree,
            event_sink,
            style_cache,
            image_cache,
        };

        self.element().draw(self.element_state().as_mut(), &mut cx);

        cx.node.draw();
    }
}
