use std::sync::Arc;

use ori_graphics::{math::Vec2, Color, Curve, Quad};
use ori_reactive::{Event, OwnedSignal};

use crate::{
    AvailableSpace, BorderRadius, BorderWidth, Context, DrawContext, EventContext, Key,
    LayoutContext, PointerEvent, StateView, Style, Styled, Unit,
};

type OnPointerEvent = Arc<dyn Fn(&PointerEvent) + Send + Sync>;

#[derive(Clone)]
pub struct CheckBox {
    pub checked: OwnedSignal<bool>,
    pub on_press: Option<OnPointerEvent>,
    pub on_release: Option<OnPointerEvent>,
    pub size: Style<Unit>,
    pub color: Style<Color>,
    pub stroke: Style<Unit>,
    pub background: Style<Color>,
    pub border_width: Style<BorderWidth>,
    pub border_radius: Style<BorderRadius>,
    pub border_color: Style<Color>,
}

impl Default for CheckBox {
    fn default() -> Self {
        Self {
            checked: OwnedSignal::new(false),
            on_press: None,
            on_release: None,
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

    pub fn new(checked: impl Into<OwnedSignal<bool>>) -> Self {
        Self {
            checked: checked.into(),
            ..Default::default()
        }
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

    pub fn stroke(mut self, stroke: impl Styled<Unit>) -> Self {
        self.stroke = stroke.style();
        self
    }

    pub fn background(mut self, background: impl Styled<Color>) -> Self {
        self.background = background.style();
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
        state: &mut CheckBoxState,
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
            handled = true;

            self.checked.set(!self.checked.get());

            if let Some(on_press) = &self.on_press {
                on_press(event);
            }
        } else if state.pressed && event.is_release() {
            state.pressed = false;
            cx.request_redraw();
            handled = true;

            if let Some(on_release) = &self.on_release {
                on_release(event);
            }
        }

        handled
    }
}

#[doc(hidden)]
#[derive(Default)]
pub struct CheckBoxState {
    pub hovered: bool,
    pub pressed: bool,
    pub transition: f32,
}

impl StateView for CheckBox {
    type State = CheckBoxState;

    fn build(&mut self, _cx: &mut Context<'_>) -> Self::State {
        CheckBoxState::default()
    }

    fn event(&mut self, state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        if event.is_handled() {
            return;
        }

        if let Some(event) = event.get::<PointerEvent>() {
            self.handle_pointer_event(state, cx, event);
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
        let border_alt = if checked {
            border.darken(0.1)
        } else {
            border.darken(0.05)
        };

        if (state.hovered || checked) && state.transition < 1.0 {
            state.transition += cx.dt() * 20.0;
            cx.request_redraw();
        } else if !state.hovered && !checked && state.transition > 0.0 {
            state.transition -= cx.dt() * 20.0;
            cx.request_redraw();
        }

        state.transition = state.transition.clamp(0.0, 1.0);

        cx.draw(Quad {
            rect: cx.rect(),
            background_color: background,
            background_image: None,
            border_radius: self.border_radius.get(cx.theme).get(cx),
            border_width: self.border_width.get(cx.theme).get(cx),
            border_color: border.mix(border_alt, state.transition),
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
