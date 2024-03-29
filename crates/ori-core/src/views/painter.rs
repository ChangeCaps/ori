use crate::{
    canvas::Canvas,
    event::Event,
    layout::{Size, Space},
    theme::Theme,
    view::{BuildCx, DrawCx, EventCx, LayoutCx, RebuildCx, View},
};

/// Create a new [`Painter`] view.
pub fn painter<T>(draw: impl FnMut(&mut DrawCx, &mut T, &mut Canvas) + 'static) -> Painter<T> {
    Painter::new(draw)
}

/// A view that draws something.
///
/// The painter takes up as much space as possible.
pub struct Painter<T> {
    /// The draw function.
    #[allow(clippy::type_complexity)]
    pub draw: Box<dyn FnMut(&mut DrawCx, &mut T, &mut Canvas)>,
}

impl<T> Painter<T> {
    /// Create a new [`Painter`] view.
    pub fn new(mut draw: impl FnMut(&mut DrawCx, &mut T, &mut Canvas) + 'static) -> Self {
        let mut snapshot = Theme::snapshot();

        Self {
            draw: Box::new(move |cx, data, canvas| snapshot.as_context(|| draw(cx, data, canvas))),
        }
    }
}

impl<T> View<T> for Painter<T> {
    type State = ();

    fn build(&mut self, _cx: &mut BuildCx, _data: &mut T) -> Self::State {}

    fn rebuild(
        &mut self,
        _state: &mut Self::State,
        _cx: &mut RebuildCx,
        _data: &mut T,
        _old: &Self,
    ) {
    }

    fn event(
        &mut self,
        _state: &mut Self::State,
        _cx: &mut EventCx,
        _data: &mut T,
        _event: &Event,
    ) {
    }

    fn layout(
        &mut self,
        _state: &mut Self::State,
        _cx: &mut LayoutCx,
        _data: &mut T,
        space: Space,
    ) -> Size {
        space.min
    }

    fn draw(
        &mut self,
        _state: &mut Self::State,
        cx: &mut DrawCx,
        data: &mut T,
        canvas: &mut Canvas,
    ) {
        (self.draw)(cx, data, canvas);
    }
}
