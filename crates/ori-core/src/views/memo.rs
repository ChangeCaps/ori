use crate::{
    context::{BuildCx, DrawCx, EventCx, LayoutCx, RebuildCx},
    event::Event,
    layout::{Size, Space},
    view::View,
};

/// Create a new [`Memo`].
pub fn memo<T, V: View<T>, D: PartialEq>(
    data: D,
    build: impl FnOnce(&mut T) -> V + 'static,
) -> Memo<T, V, D> {
    Memo::new(data, build)
}

/// A view that only builds the inner view when certain data changes.
pub struct Memo<T, V, D> {
    data: Option<D>,

    #[allow(clippy::type_complexity)]
    build: Option<Box<dyn FnOnce(&mut T) -> V>>,
}

impl<T, V: View<T>, D: PartialEq> Memo<T, V, D> {
    /// Create a new [`Memo`].
    pub fn new(data: D, build: impl FnOnce(&mut T) -> V + 'static) -> Self {
        Self {
            data: Some(data),
            build: Some(Box::new(build)),
        }
    }

    fn build(&mut self, data: &mut T) -> V {
        (self.build.take().expect("Memo::build called twice"))(data)
    }
}

#[doc(hidden)]
pub struct MemoState<T, V: View<T>, D> {
    view: V,
    state: V::State,
    data: Option<D>,
}

impl<T, V: View<T>, D: PartialEq> View<T> for Memo<T, V, D> {
    type State = MemoState<T, V, D>;

    fn build(&mut self, cx: &mut BuildCx, data: &mut T) -> Self::State {
        let mut view = self.build(data);
        let state = view.build(cx, data);
        let data = self.data.take();

        MemoState { view, state, data }
    }

    fn rebuild(&mut self, state: &mut Self::State, cx: &mut RebuildCx, data: &mut T, _old: &Self) {
        if self.data != state.data {
            let mut view = self.build(data);
            view.rebuild(&mut state.state, cx, data, &state.view);

            state.view = view;
            state.data = self.data.take();
        }
    }

    fn event(
        &mut self,
        state: &mut Self::State,
        cx: &mut EventCx,
        data: &mut T,
        event: &Event,
    ) -> bool {
        state.view.event(&mut state.state, cx, data, event)
    }

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut LayoutCx,
        data: &mut T,
        space: Space,
    ) -> Size {
        state.view.layout(&mut state.state, cx, data, space)
    }

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawCx, data: &mut T) {
        state.view.draw(&mut state.state, cx, data);
    }
}
