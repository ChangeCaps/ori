use std::sync::Arc;

use glam::Vec2;
use ori_graphics::{Color, Quad};
use ori_reactive::Event;

use crate::{
    AvailableSpace, DrawContext, EventContext, IntoView, LayoutContext, Node, Padding,
    PointerEvent, StateView,
};

type OnPointerEvent = Arc<dyn Fn(&PointerEvent) + Send + Sync>;

#[derive(Clone)]
pub struct Button {
    content: Node,
    on_press: Option<OnPointerEvent>,
    on_release: Option<OnPointerEvent>,
    padding: Padding,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            content: Default::default(),
            on_press: None,
            on_release: None,
            padding: Padding::uniform(5.0),
        }
    }
}

impl Button {
    pub fn new(content: impl IntoView) -> Self {
        Self {
            content: Node::new(content),
            ..Default::default()
        }
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

    fn handle_pointer_event(
        &self,
        state: &mut ButtonState,
        cx: &mut EventContext<'_>,
        event: &PointerEvent,
    ) -> bool {
        let local = cx.local(event.position);
        let mut handled = false;

        state.hovered = cx.rect().contains(local);

        if state.hovered && event.is_press() {
            state.pressed = true;
            cx.request_redraw();

            if let Some(ref on_press) = self.on_press {
                on_press(event);
                handled = true;
            }
        } else if event.is_release() {
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
}

impl StateView for Button {
    type State = ButtonState;

    fn build(&self) -> Self::State {
        Default::default()
    }

    fn event(&self, state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        cx.with_padding(self.padding, |cx| {
            cx.child(0, &self.content, event);
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
        &self,
        _state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        cx.child(0, &self.content, space.pad(self.padding)) + self.padding
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        let color = if state.hovered {
            if state.pressed {
                Color::BLUE
            } else {
                Color::GREEN
            }
        } else {
            Color::CYAN
        };

        cx.draw(Quad {
            rect: cx.rect(),
            background_color: color,
            background_image: None,
            border_radius: [5.0; 4],
            ..Default::default()
        });

        cx.with_padding(self.padding, |cx| {
            cx.child(0, &self.content);
        });
    }
}
