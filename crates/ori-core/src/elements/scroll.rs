use glam::Vec2;
use ori_graphics::{Quad, Rect};
use ori_macro::Build;
use ori_reactive::Event;
use ori_style::Style;

use crate::{
    AvailableSpace, Axis, Children, Context, DrawContext, Element, EventContext, FlexLayout,
    LayoutContext, PointerEvent,
};

/// A scroll element.
#[derive(Default, Build)]
pub struct Scroll {
    /// The children of the scroll.
    #[children]
    pub children: Children,
}

impl Scroll {
    /// Create a new scroll.
    pub fn new() -> Self {
        Self::default()
    }

    fn scrollbar_rect(&self, state: &ScrollState, cx: &mut impl Context) -> Rect {
        let axis = cx.style::<Axis>("direction");
        let max_width = axis.major(cx.rect().size());

        let width = cx.style_range("scrollbar-width", 0.0..max_width);
        let padding = cx.style_range("scrollbar-padding", 0.0..max_width - width);

        let max_height = axis.minor(cx.rect().size()) - padding * 2.0;
        let height = cx.style_range("scrollbar-height", 0.0..max_height);

        let scrollbar_size = axis.pack(height, width);
        let range = axis.major(cx.rect().size()) - height - padding * 2.0;

        Rect::min_size(
            axis.pack(
                axis.major(cx.rect().min) + range * axis.major(state.scroll) + padding,
                axis.minor(cx.rect().max) - axis.minor(scrollbar_size) - padding,
            ),
            scrollbar_size,
        )
    }

    fn scrollbar_track_rect(&self, cx: &mut impl Context) -> Rect {
        let axis = cx.style::<Axis>("direction");

        let max_width = axis.major(cx.rect().size());
        let width = cx.style_range("scrollbar-width", 0.0..max_width);

        let padding = cx.style_range("scrollbar-padding", 0.0..max_width - width);

        Rect::min_size(
            axis.pack(
                axis.major(cx.rect().min) + padding,
                axis.minor(cx.rect().max) - width - padding,
            ),
            axis.pack(axis.major(cx.rect().size()) - padding * 2.0, width),
        )
    }

    fn overflow(&self, cx: &mut impl Context) -> Vec2 {
        self.children.size() - cx.size()
    }

    fn should_show_scrollbar(&self, cx: &mut impl Context) -> bool {
        self.overflow(cx).max_element() > 1.0
    }

    fn handle_pointer_event(
        &self,
        state: &mut ScrollState,
        cx: &mut EventContext,
        event: &PointerEvent,
    ) -> bool {
        let mut handled = false;

        let axis = cx.style::<Axis>("direction");

        if event.scroll_delta != Vec2::ZERO && cx.hovered() {
            let overflow = self.overflow(cx);
            state.scroll -= axis.pack(event.scroll_delta.y, 0.0) / overflow * 10.0;
            state.scroll = state.scroll.clamp(Vec2::ZERO, Vec2::ONE);

            cx.request_redraw();

            handled = true;
        }

        if !self.should_show_scrollbar(cx) {
            return handled;
        }

        let scrollbar_track_rect = self.scrollbar_track_rect(cx);

        if scrollbar_track_rect.contains(event.position) {
            handled = true;
            cx.hover();
        }

        if scrollbar_track_rect.contains(event.position) && event.is_press() {
            cx.activate();
        }

        if event.is_release() {
            cx.deactivate();
        }

        if cx.active() {
            let start = axis.major(scrollbar_track_rect.min);
            let end = axis.major(scrollbar_track_rect.max);
            let range = end - start;

            let scroll = (axis.major(event.position) - start) / range;
            state.scroll = axis.pack(scroll.clamp(0.0, 1.0), 0.0);

            cx.request_redraw();

            handled = true;
        }

        handled
    }
}

/// The state of a scroll element.
#[derive(Default)]
pub struct ScrollState {
    scroll: Vec2,
}

impl Element for Scroll {
    type State = ScrollState;

    fn build(&self) -> Self::State {
        ScrollState::default()
    }

    fn style(&self) -> Style {
        Style::new("scroll")
    }

    fn event(&self, state: &mut Self::State, cx: &mut EventContext, event: &Event) {
        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if self.handle_pointer_event(state, cx, pointer_event) {
                event.handle();
            }
        }

        self.children.event(cx, event);
    }

    fn layout(&self, _: &mut Self::State, cx: &mut LayoutContext, space: AvailableSpace) -> Vec2 {
        let flex = FlexLayout {
            axis: cx.style::<Axis>("direction"),
            justify_content: cx.style("justify-content"),
            align_items: cx.style("align-items"),
            gap: cx.style_range("gap", 0.0..space.max.min_element() / 2.0),
            ..Default::default()
        };

        let size = self.children.flex_layout(cx, space, flex);

        space.constrain(size)
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext) {
        cx.draw_quad();

        let overflow = self.overflow(cx);
        let padding = cx.padding().top_left();
        self.children.set_offset(-state.scroll * overflow + padding);

        let container_rect = cx.rect().translate(padding);
        cx.layer().clip(container_rect).draw(|cx| {
            self.children.draw(cx);
        });

        if !self.should_show_scrollbar(cx) {
            return;
        }

        // draw scrollbar track
        let rect = self.scrollbar_track_rect(cx);

        let max_radius = rect.size().min_element() / 2.0;
        let radius = cx.style_range("scrollbar-border-radius", 0.0..max_radius);

        let quad = Quad {
            rect,
            background: cx.style("scrollbar-track-color"),
            border_radius: [radius; 4],
            border_width: cx.style_range("scrollbar-track-border-width", 0.0..max_radius),
            border_color: cx.style("scrollbar-track-border-color"),
        };

        cx.layer().z_index(100.0).draw(|cx| {
            cx.draw(quad);
        });

        // draw scrollbar
        let rect = self.scrollbar_rect(state, cx);

        let quad = Quad {
            rect,
            background: cx.style("scrollbar-color"),
            border_radius: [radius; 4],
            border_width: cx.style_range("scrollbar-border-width", 0.0..max_radius),
            border_color: cx.style("scrollbar-border-color"),
        };

        cx.layer().z_index(100.0).draw(|cx| {
            cx.draw(quad);
        });
    }
}
