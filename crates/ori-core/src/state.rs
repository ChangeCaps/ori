use glam::Vec2;
use ori_reactive::Event;

use crate::{AvailableSpace, DrawContext, EventContext, LayoutContext, Tree, View};

#[allow(unused_variables)]
pub trait StateView: Send + Sync + 'static {
    type State: Send + Sync + 'static;

    fn build(&self) -> Self::State;

    fn event(&self, state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {}

    fn layout(
        &self,
        state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        space.min
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext<'_>) {}
}

fn take_state<V: StateView>(view: &V, tree: &mut Tree) -> Box<V::State> {
    let Some(state) = tree.take_view_state() else {
        return Box::new(view.build());
    };

    match state.downcast::<V::State>() {
        Ok(state) => state,
        Err(_) => {
            tree.children.clear();
            Box::new(view.build())
        }
    }
}

impl<T: StateView> View for T {
    fn event(&self, cx: &mut EventContext<'_>, event: &Event) {
        let mut state = take_state(self, cx.tree);
        self.event(&mut state, cx, event);
        cx.tree.set_view_state(state);
    }

    fn layout(&self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        let mut state = take_state(self, cx.tree);
        let size = self.layout(&mut state, cx, space);
        cx.tree.set_view_state(state);

        size
    }

    fn draw(&self, cx: &mut DrawContext<'_>) {
        let mut state = take_state(self, cx.tree);
        self.draw(&mut state, cx);
        cx.tree.set_view_state(state);
    }
}
