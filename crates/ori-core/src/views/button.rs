use ori_graphics::{math::Vec2, Color, Quad};
use ori_reactive::Event;

use crate::{
    AvailableSpace, BorderRadius, BorderWidth, Context, Cursor, DrawContext, EventContext, Key,
    LayoutContext, Node, Padding, PointerEvent, StateView, Style, Styled, Transition, Unit,
};

use super::EventCallback;

/// A button view.
pub struct Button {
    /// The content of the button.
    pub content: Node,
    /// The event to fire when the button is pressed.
    pub on_press: Option<EventCallback<PointerEvent>>,
    /// The event to fire when the button is released.
    pub on_release: Option<EventCallback<PointerEvent>>,
    /// The time it takes for the button to transition between states.
    pub trans: Transition,
    /// The padding of the button.
    pub padding: Padding,
    /// The is the distance the button moves up when hovered.
    pub float: Option<Style<Unit>>,
    /// The color of the button.
    pub color: Style<Color>,
    /// The color of the button when hovered.
    pub hover_color: Option<Style<Color>>,
    /// The border width of the button.
    pub border_width: Style<BorderWidth>,
    /// The border radius of the button.
    pub border_radius: Style<BorderRadius>,
    /// The border color of the button.
    pub border_color: Style<Color>,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            content: Default::default(),
            on_press: None,
            on_release: None,
            trans: Transition::smooth(0.1),
            padding: Padding::all(Unit::Em(0.5)),
            float: None,
            color: Style::new(Self::COLOR),
            hover_color: None,
            border_width: Style::new(Self::BORDER_WIDTH),
            border_radius: Style::new(Self::BORDER_RADIUS),
            border_color: Style::new(Self::BORDER_COLOR),
        }
    }
}

impl Button {
    pub const FLOAT: Key<Unit> = Key::new("button.float");
    pub const COLOR: Key<Color> = Key::new("button.color");
    pub const BORDER_WIDTH: Key<BorderWidth> = Key::new("button.border-width");
    pub const BORDER_RADIUS: Key<BorderRadius> = Key::new("button.border-radius");
    pub const BORDER_COLOR: Key<Color> = Key::new("button.border-color");

    /// Create a new button view.
    pub fn new(content: impl Into<Node>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Creates a new fancy button.
    pub fn fancy(content: impl Into<Node>) -> Self {
        Self::new(content).float(Style::new(Self::FLOAT))
    }

    /// Set the on press callback of the button.
    pub fn on_press(mut self, on_press: impl Fn(&PointerEvent) + Send + 'static) -> Self {
        self.on_press = Some(Box::new(on_press));
        self
    }

    /// Set the on release callback of the button.
    pub fn on_release(mut self, on_release: impl Fn(&PointerEvent) + Send + 'static) -> Self {
        self.on_release = Some(Box::new(on_release));
        self
    }

    /// Set the transition time of the button.
    pub fn transition(mut self, transition: impl Into<Transition>) -> Self {
        self.trans = transition.into();
        self
    }

    /// Set the floating distance of the button.
    pub fn float(mut self, floating: impl Styled<Unit>) -> Self {
        self.float = Some(floating.style());
        self
    }

    /// Set the padding of the button.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Set the color of the button.
    pub fn color(mut self, color: impl Styled<Color>) -> Self {
        self.color = color.style();
        self
    }

    /// Set the hover color of the button.
    pub fn hover_color(mut self, hover_color: impl Into<Option<Style<Color>>>) -> Self {
        self.hover_color = hover_color.into();
        self
    }

    /// Set the border width of the button.
    pub fn border_width(mut self, border_width: impl Styled<BorderWidth>) -> Self {
        self.border_width = border_width.style();
        self
    }

    /// Set the border radius of the button.
    pub fn border_radius(mut self, border_radius: impl Styled<BorderRadius>) -> Self {
        self.border_radius = border_radius.style();
        self
    }

    /// Set the border color of the button.
    pub fn border_color(mut self, border_color: impl Styled<Color>) -> Self {
        self.border_color = border_color.style();
        self
    }

    fn handle_pointer_event(
        &mut self,
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

            if state.hovered {
                cx.set_cursor(Some(Cursor::Pointer));
            } else {
                cx.set_cursor(None);
            }
        }

        if state.hovered && event.is_press() {
            state.pressed = true;
            cx.request_redraw();

            if let Some(ref mut on_press) = self.on_press {
                on_press(event);
                handled = true;
            }
        } else if state.pressed && event.is_release() {
            state.pressed = false;
            cx.request_redraw();

            if let Some(ref mut on_release) = self.on_release {
                on_release(event);
                handled = true;
            }
        }

        handled
    }

    fn float_offset(&self, state: &ButtonState, cx: &Context<'_>) -> Vec2 {
        match self.float {
            Some(ref float) => {
                let float = float.get(cx.theme).get(cx);
                Vec2::new(0.0, -self.trans.get(state.trans) * float)
            }
            None => Vec2::ZERO,
        }
    }

    fn translation(&self, state: &ButtonState, cx: &Context<'_>) -> Vec2 {
        let float = self.float_offset(state, cx);
        self.padding.offset(cx) + float
    }
}

#[doc(hidden)]
#[derive(Default)]
pub struct ButtonState {
    pub pressed: bool,
    pub hovered: bool,
    pub trans: f32,
}

impl StateView for Button {
    type State = ButtonState;

    fn build(&mut self, _cx: &mut Context<'_>) -> Self::State {
        Default::default()
    }

    fn event(&mut self, state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        cx.with_translation(self.translation(state, cx), |cx| {
            self.content.event(cx, event);
        });

        if event.is_handled() {
            return;
        }

        if let Some(pointer) = event.get::<PointerEvent>() {
            if self.handle_pointer_event(state, cx, pointer) {
                event.handle();
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

        let active = state.hovered && !state.pressed;
        if self.trans.update(&mut state.trans, active, cx.dt()) {
            cx.request_redraw();
        }

        if self.float.is_some() && state.trans > 0.0 {
            cx.draw(Quad {
                rect: cx.rect(),
                background_color: color.darken(0.1),
                background_image: None,
                border_radius: self.border_radius.get(cx.theme).get(cx),
                border_width: self.border_width.get(cx.theme).get(cx),
                border_color: self.border_color.get(cx.theme),
            });
        }

        cx.draw(Quad {
            rect: cx.rect() + self.float_offset(state, cx),
            background_color: color.mix(hover, state.trans),
            background_image: None,
            border_radius: self.border_radius.get(cx.theme).get(cx),
            border_width: self.border_width.get(cx.theme).get(cx),
            border_color: self.border_color.get(cx.theme),
        });

        cx.with_translation(self.translation(state, cx), |cx| {
            self.content.draw(cx);
        });
    }
}
