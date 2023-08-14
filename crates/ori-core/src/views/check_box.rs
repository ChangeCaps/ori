use ori_graphics::{math::Vec2, Color, Curve, Quad};
use ori_reactive::{Event, OwnedSignal};

use crate::{
    AvailableSpace, BorderRadius, BorderWidth, Context, DrawContext, EventContext, Key,
    LayoutContext, PointerEvent, StateView, Style, Styled, Transition, Unit,
};

use super::EventCallback;

/// A check box view.
pub struct CheckBox {
    /// The whether the box is checked.
    pub checked: OwnedSignal<bool>,
    /// The event to fire when the box is pressed.
    pub on_press: Option<EventCallback<PointerEvent>>,
    /// The event to fire when the box is released.
    pub on_release: Option<EventCallback<PointerEvent>>,
    /// The transition of the box.
    pub trans: Transition,
    /// The size of the box.
    pub size: Style<Unit>,
    /// The color of the check mark.
    pub color: Style<Color>,
    /// The stroke thickness of the check mark.
    pub stroke: Style<Unit>,
    /// The background color of the box.
    pub background: Style<Color>,
    /// The border width of the box.
    pub border_width: Style<BorderWidth>,
    /// The border radius of the box.
    pub border_radius: Style<BorderRadius>,
    /// The border color of the box.
    pub border_color: Style<Color>,
}

impl Default for CheckBox {
    fn default() -> Self {
        Self {
            checked: OwnedSignal::new(false),
            on_press: None,
            on_release: None,
            trans: Transition::smooth(0.1),
            size: Style::new(Self::SIZE),
            color: Style::new(Self::COLOR),
            stroke: Style::new(Self::STROKE),
            background: Style::new(Self::BACKGROUND),
            border_width: Style::new(Self::BORDER_WIDTH),
            border_radius: Style::new(Self::BORDER_RADIUS),
            border_color: Style::new(Self::BORDER_COLOR),
        }
    }
}

impl CheckBox {
    pub const SIZE: Key<Unit> = Key::new("check-box.size");
    pub const COLOR: Key<Color> = Key::new("check-box.color");
    pub const STROKE: Key<Unit> = Key::new("check-box.stroke");
    pub const BACKGROUND: Key<Color> = Key::new("check-box.background");
    pub const BORDER_WIDTH: Key<BorderWidth> = Key::new("check-box.border-width");
    pub const BORDER_RADIUS: Key<BorderRadius> = Key::new("check-box.border-radius");
    pub const BORDER_COLOR: Key<Color> = Key::new("check-box.border-color");

    /// Create a new check box.
    pub fn new(checked: impl Into<OwnedSignal<bool>>) -> Self {
        Self {
            checked: checked.into(),
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

    /// Set the transition of the box.
    pub fn transition(mut self, transition: impl Into<Transition>) -> Self {
        self.trans = transition.into();
        self
    }

    /// Set the size of the box.
    pub fn size(mut self, size: impl Styled<Unit>) -> Self {
        self.size = size.style();
        self
    }

    /// Set the color of the check mark.
    pub fn color(mut self, color: impl Styled<Color>) -> Self {
        self.color = color.style();
        self
    }

    /// Set the stroke thickness of the check mark.
    pub fn stroke(mut self, stroke: impl Styled<Unit>) -> Self {
        self.stroke = stroke.style();
        self
    }

    /// Set the background color of the box.
    pub fn background(mut self, background: impl Styled<Color>) -> Self {
        self.background = background.style();
        self
    }

    /// Set the border width of the box.
    pub fn border_width(mut self, border_width: impl Styled<BorderWidth>) -> Self {
        self.border_width = border_width.style();
        self
    }

    /// Set the border radius of the box.
    pub fn border_radius(mut self, border_radius: impl Styled<BorderRadius>) -> Self {
        self.border_radius = border_radius.style();
        self
    }

    /// Set the border color of the box.
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
            cx.request_redraw();
        }

        if cx.is_hovered() && event.is_press() {
            cx.set_active(true);
            cx.request_redraw();
            handled = true;

            self.checked.set(!self.checked.get());

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
pub struct CheckBoxState {
    pub t: f32,
}

impl StateView for CheckBox {
    type State = CheckBoxState;

    fn build(&mut self, _cx: &mut Context<'_>) -> Self::State {
        CheckBoxState::default()
    }

    fn event(&mut self, _state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        if event.is_handled() {
            return;
        }

        if let Some(event) = event.get::<PointerEvent>() {
            self.handle_pointer_event(cx, event);
        }
    }

    fn layout(
        &mut self,
        _state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        let size = self.size.get(cx.theme).get(cx);
        space.fit(Vec2::splat(size))
    }

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        let checked = self.checked.get();

        let color = self.color.get(cx.theme);
        let background = self.background.get(cx.theme);
        let border = self.border_color.get(cx.theme);
        let border_alt = border.brighten(0.2);

        if self.trans.update(&mut state.t, cx.is_hovered(), cx.dt()) {
            cx.request_redraw();
        }

        cx.draw(Quad {
            rect: cx.rect(),
            background_color: background,
            background_image: None,
            border_radius: self.border_radius.get(cx.theme).get(cx),
            border_width: self.border_width.get(cx.theme).get(cx),
            border_color: border.mix(border_alt, self.trans.get(state.t)),
        });

        if checked {
            let mut curve = Curve::new();
            curve.add_point(Vec2::new(0.2, 0.5) * cx.size());
            curve.add_point(Vec2::new(0.4, 0.7) * cx.size());
            curve.add_point(Vec2::new(0.8, 0.3) * cx.size());

            let stroke = self.stroke.get(cx.theme).get(cx);
            cx.draw(curve.stroke(stroke, color));
        }
    }
}
