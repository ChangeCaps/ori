use ori_graphics::math::Vec2;
use ori_reactive::Event;

use crate::{AvailableSpace, Context, DrawContext, EventContext, LayoutContext, View};

#[allow(unused_variables)]
pub trait StateView: Send + 'static {
    type State: Send + 'static;

    fn build(&mut self, cx: &mut Context<'_>) -> Self::State;

    fn event(&mut self, state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event);

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2;

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawContext<'_>);
}

fn take_state<V: StateView>(view: &mut V, cx: &mut Context<'_>) -> Box<V::State> {
    let Some(state) = cx.view_state.take_state() else {
        return Box::new(view.build(cx));
    };

    match state.downcast::<V::State>() {
        Ok(state) => state,
        Err(_) => Box::new(view.build(cx)),
    }
}

impl<T: StateView> View for T {
    fn event(&mut self, cx: &mut EventContext<'_>, event: &Event) {
        let mut state = take_state(self, cx);
        self.event(&mut state, cx, event);
        cx.view_state.set_state(state);
    }

    fn layout(&mut self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        let mut state = take_state(self, cx);
        let size = self.layout(&mut state, cx, space);
        cx.view_state.set_state(state);

        size
    }

    fn draw(&mut self, cx: &mut DrawContext<'_>) {
        let mut state = take_state(self, cx);
        self.draw(&mut state, cx);
        cx.view_state.set_state(state);
    }
}
