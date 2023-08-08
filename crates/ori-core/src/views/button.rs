use glam::Vec2;
use ori_graphics::{Color, Quad};
use ori_reactive::Event;

use crate::{
    AvailableSpace, DrawContext, EventContext, IntoView, LayoutContext, Node, Padding,
    PointerEvent, StateView,
};

type On<T> = Box<dyn Fn(T) + Send + Sync>;

pub struct Button {
    content: Node,
    on_press: Option<On<PointerEvent>>,
    on_release: Option<On<PointerEvent>>,
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

    pub fn on_press(mut self, on_press: impl Fn(PointerEvent) + Send + Sync + 'static) -> Self {
        self.on_press = Some(Box::new(on_press));
        self
    }

    pub fn on_release(mut self, on_release: impl Fn(PointerEvent) + Send + Sync + 'static) -> Self {
        self.on_release = Some(Box::new(on_release));
        self
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
            let local = cx.local(pointer.position);

            state.hovered = cx.rect().contains(local);

            if state.hovered && pointer.is_press() {
                state.pressed = true;
                event.handle();

                if let Some(ref on_press) = self.on_press {
                    on_press(pointer.clone());
                }
            } else if pointer.is_release() {
                state.pressed = false;
                event.handle();

                if let Some(ref on_release) = self.on_release {
                    on_release(pointer.clone());
                }
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

    fn draw(&self, _state: &mut Self::State, cx: &mut DrawContext<'_>) {
        cx.draw(Quad {
            rect: cx.rect(),
            background_color: Color::CYAN,
            background_image: None,
            border_radius: [5.0; 4],
            border_width: [1.0; 4],
            border_color: Color::BLACK,
        });

        cx.with_padding(self.padding, |cx| {
            cx.child(0, &self.content);
        });
    }
}
