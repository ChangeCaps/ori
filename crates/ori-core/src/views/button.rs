use std::sync::Arc;

use ori_graphics::{math::Vec2, Color, Quad};
use ori_reactive::Event;

use crate::{
    AvailableSpace, BorderRadius, BorderWidth, Context, DrawContext, EventContext, Key,
    LayoutContext, Node, Padding, PointerEvent, StateView, Style, Styled, Unit,
};

type OnPointerEvent = Arc<dyn Fn(&PointerEvent) + Send + Sync>;

pub struct Button {
    pub content: Node,
    pub on_press: Option<OnPointerEvent>,
    pub on_release: Option<OnPointerEvent>,
    pub transition_time: f32,
    pub padding: Padding,
    pub color: Style<Color>,
    pub hover_color: Option<Style<Color>>,
    pub border_width: Style<BorderWidth>,
    pub border_radius: Style<BorderRadius>,
    pub border_color: Style<Color>,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            content: Default::default(),
            on_press: None,
            on_release: None,
            transition_time: 0.05,
            padding: Padding::all(Unit::Em(0.5)),
            color: Style::new(Self::COLOR),
            hover_color: None,
            border_width: Style::new(Self::BORDER_WIDTH),
            border_radius: Style::new(Self::BORDER_RADIUS),
            border_color: Style::new(Self::BORDER_COLOR),
        }
    }
}

impl Button {
    pub const COLOR: Key<Color> = Key::new("button.color");
    pub const BORDER_WIDTH: Key<BorderWidth> = Key::new("button.border-width");
    pub const BORDER_RADIUS: Key<BorderRadius> = Key::new("button.border-radius");
    pub const BORDER_COLOR: Key<Color> = Key::new("button.border-color");

    pub fn new(content: impl Into<Node>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn transition_time(mut self, transition_time: f32) -> Self {
        self.transition_time = transition_time;
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn on_press(mut self, on_press: impl Fn(&PointerEvent) + Send + Sync + 'static) -> Self {
        self.on_press = Some(Arc::new(on_press));
        self
    }

    pub fn on_release(
        mut self,
        on_release: impl Fn(&PointerEvent) + Send + Sync + 'static,
    ) -> Self {
        self.on_release = Some(Arc::new(on_release));
        self
    }

    pub fn color(mut self, color: impl Styled<Color>) -> Self {
        self.color = color.style();
        self
    }

    pub fn hover_color(mut self, hover_color: impl Into<Option<Style<Color>>>) -> Self {
        self.hover_color = hover_color.into();
        self
    }

    pub fn border_width(mut self, border_width: impl Styled<BorderWidth>) -> Self {
        self.border_width = border_width.style();
        self
    }

    pub fn border_radius(mut self, border_radius: impl Styled<BorderRadius>) -> Self {
        self.border_radius = border_radius.style();
        self
    }

    pub fn border_color(mut self, border_color: impl Styled<Color>) -> Self {
        self.border_color = border_color.style();
        self
    }

    fn handle_pointer_event(
        &self,
        state: &mut ButtonState,
        cx: &mut EventContext<'_>,
        event: &PointerEvent,
    ) -> bool {
        let local = cx.local(event.position);
        let mut handled = false;

        let hovered = cx.rect().contains(local) && !event.left;

        if state.hovered != hovered {
            state.hovered = hovered;
            cx.request_redraw();
        }

        if state.hovered && event.is_press() {
            state.pressed = true;
            cx.request_redraw();

            if let Some(ref on_press) = self.on_press {
                on_press(event);
                handled = true;
            }
        } else if state.pressed && event.is_release() {
            state.pressed = false;
            cx.request_redraw();

            if let Some(ref on_release) = self.on_release {
                on_release(event);
                handled = true;
            }
        }

        handled
    }
}

#[doc(hidden)]
#[derive(Default)]
pub struct ButtonState {
    pub pressed: bool,
    pub hovered: bool,
    pub transition: f32,
}

impl StateView for Button {
    type State = ButtonState;

    fn build(&mut self, _cx: &mut Context<'_>) -> Self::State {
        Default::default()
    }

    fn event(&mut self, state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        cx.with_padding(self.padding, |cx| {
            self.content.event(cx, event);
        });

        if event.is_handled() {
            return;
        }

        if let Some(pointer) = event.get::<PointerEvent>() {
            if self.handle_pointer_event(state, cx, pointer) {
                event.handled();
            }
        }
    }

    fn layout(
        &mut self,
        _state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        self.content.layout_padded(cx, space, self.padding)
    }

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        let color = self.color.get(cx.theme);
        let hover = match self.hover_color {
            Some(ref hover_color) => hover_color.get(cx.theme),
            None => color.brighten(0.05),
        };

        if state.hovered && state.transition < 1.0 {
            cx.request_redraw();
            state.transition += cx.dt() / self.transition_time;
        } else if !state.hovered && state.transition > 0.0 {
            cx.request_redraw();
            state.transition -= cx.dt() / self.transition_time;
        }

        state.transition = state.transition.clamp(0.0, 1.0);

        cx.draw(Quad {
            rect: cx.rect(),
            background_color: color.mix(hover, state.transition),
            background_image: None,
            border_radius: self.border_radius.get(cx.theme).get(cx),
            border_width: self.border_width.get(cx.theme).get(cx),
            border_color: self.border_color.get(cx.theme),
        });

        cx.with_padding(self.padding, |cx| {
            self.content.draw(cx);
        });
    }
}
