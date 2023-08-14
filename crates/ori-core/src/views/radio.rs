use ori_graphics::{math::Vec2, Color, Quad, Rect};
use ori_reactive::{Event, OwnedSignal};

use crate::{
    AvailableSpace, Context, DrawContext, EventContext, Key, LayoutContext, PointerEvent,
    StateView, Style, Styled, Transition, Unit,
};

use super::EventCallback;

/// A radio button view.
pub struct Radio {
    /// The content of the button.
    pub selected: OwnedSignal<bool>,
    /// The event to fire when the button is pressed.
    pub on_press: Option<EventCallback<PointerEvent>>,
    /// The event to fire when the button is released.
    pub on_release: Option<EventCallback<PointerEvent>>,
    /// The transition of the button.
    pub trans: Transition,
    /// The radius of the button.
    pub radius: Style<Unit>,
    /// The color of the button.
    pub color: Style<Color>,
    /// The background color of the button.
    pub background: Style<Color>,
    /// The border width of the button.
    pub border_width: Style<Unit>,
    /// The border color of the button.
    pub border_color: Style<Color>,
}

impl Default for Radio {
    fn default() -> Self {
        Self {
            selected: OwnedSignal::new(false),
            on_press: None,
            on_release: None,
            trans: Transition::smooth(0.1),
            radius: Style::new(Self::RADIUS),
            color: Style::new(Self::COLOR),
            background: Style::new(Self::BACKGROUND),
            border_width: Style::new(Self::BORDER_WIDTH),
            border_color: Style::new(Self::BORDER_COLOR),
        }
    }
}

impl Radio {
    pub const RADIUS: Key<Unit> = Key::new("radio.radius");
    pub const COLOR: Key<Color> = Key::new("radio.color");
    pub const BACKGROUND: Key<Color> = Key::new("radio.background");
    pub const BORDER_WIDTH: Key<Unit> = Key::new("radio.border-width");
    pub const BORDER_COLOR: Key<Color> = Key::new("radio.border-color");

    /// Create a new radio button.
    pub fn new(selected: impl Into<OwnedSignal<bool>>) -> Self {
        Self {
            selected: selected.into(),
            ..Default::default()
        }
    }

    /// Set the on press callback.
    pub fn on_press(mut self, on_press: impl FnMut(&PointerEvent) + Send + 'static) -> Self {
        self.on_press = Some(Box::new(on_press));
        self
    }

    /// Set the on release callback.
    pub fn on_release(mut self, on_release: impl FnMut(&PointerEvent) + Send + 'static) -> Self {
        self.on_release = Some(Box::new(on_release));
        self
    }

    /// Set the transition of the button.
    pub fn transition(mut self, transition: impl Into<Transition>) -> Self {
        self.trans = transition.into();
        self
    }

    /// Set the radius of the button.
    pub fn radius(mut self, radius: impl Styled<Unit>) -> Self {
        self.radius = radius.style();
        self
    }

    /// Set the color of the button.
    pub fn color(mut self, color: impl Styled<Color>) -> Self {
        self.color = color.style();
        self
    }

    /// Set the background color of the button.
    pub fn background(mut self, background: impl Styled<Color>) -> Self {
        self.background = background.style();
        self
    }

    /// Set the border width of the button.
    pub fn border_width(mut self, border_width: impl Styled<Unit>) -> Self {
        self.border_width = border_width.style();
        self
    }

    /// Set the border color of the button.
    pub fn border_color(mut self, border_color: impl Styled<Color>) -> Self {
        self.border_color = border_color.style();
        self
    }

    fn handle_pointer_event(&mut self, cx: &mut EventContext<'_>, event: &PointerEvent) -> bool {
        let local = cx.local(event.position);
        let mut handled = false;

        let hovered = cx.rect().contains(local) && !event.left;

        if cx.is_hovered() != hovered {
            cx.set_hovered(hovered);
            handled = true;
        }

        if cx.is_hovered() && event.is_press() {
            cx.set_active(true);
            cx.request_redraw();
            handled = true;

            if let Some(ref mut on_press) = self.on_press {
                on_press(event);
            }
        } else if cx.is_active() && event.is_release() {
            cx.set_active(false);
            cx.request_redraw();
            handled = true;

            if let Some(ref mut on_release) = self.on_release {
                on_release(event);
            }
        }

        handled
    }
}

#[doc(hidden)]
#[derive(Default)]
pub struct RadioState {
    t: f32,
}

impl StateView for Radio {
    type State = RadioState;

    fn build(&mut self, _cx: &mut Context<'_>) -> Self::State {
        RadioState::default()
    }

    fn event(&mut self, _state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        if event.is_handled() {
            return;
        }

        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if self.handle_pointer_event(cx, pointer_event) {
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
        let radius = self.radius.get(cx.theme).get(cx);
        let size = Vec2::splat(radius * 2.0);
        space.fit(size)
    }

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        let selected = self.selected.get();

        let color = self.color.get(cx.theme);
        let background = self.background.get(cx.theme);
        let border = self.border_color.get(cx.theme);
        let border_alt = border.brighten(0.2);

        if self.trans.update(&mut state.t, cx.is_hovered(), cx.dt()) {
            cx.request_redraw();
        }

        let radius = self.radius.get(cx.theme).get(cx);
        let border_width = self.border_width.get(cx.theme).get(cx);

        cx.draw(Quad {
            rect: cx.rect(),
            background_color: background,
            background_image: None,
            border_radius: [radius; 4],
            border_width: [border_width; 4],
            border_color: border.mix(border_alt, self.trans.get(state.t)),
        });

        if selected {
            let inner_radius = radius / 2.0;
            let size = cx.size();

            cx.draw(Quad {
                rect: Rect::min_size(size / 4.0, size / 2.0),
                background_color: color,
                background_image: None,
                border_radius: [inner_radius; 4],
                border_width: [0.0; 4],
                border_color: Color::TRANSPARENT,
            });
        }
    }
}
