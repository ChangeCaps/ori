use crate::{BuildCx, Canvas, DrawCx, Event, EventCx, LayoutCx, RebuildCx, Size, Space};

/// A single UI component.
///
/// This trait is implemented by all UI components. The user interface is built
/// by composing these components into a `view-tree`. This operation should be
/// fast, as it is performed very often.
///
/// A view also has an associated `state` type, that is persistent across `view-trees`.
/// When calling [`View::build`], the view will build it's state. A view containing
/// another view must also store it's child's state. This is usually done by wrapping
/// it in a tuple `(MyState, ContentState)`.
///
/// In case a view contains another view the contents should always be wrapped in
/// either [`Content`] or [`ContentSequence`]. If this is not done strange issues
/// are _very_ likely to occur.
///
/// [`View`] has four primary methods:
/// - [`View::rebuild`] is called after a new `view-tree` has been built, on the
///     new tree. The view can then compare itself to the old tree and update it's
///     state accordingly. When a view differs from the old tree, it should call
///     [`RebuildCx::request_layout`] or [`RebuildCx::request_draw`] when applicable.
///     This can be quite tedius to write out, so the [`Rebuild`] derive macro can be
///     used to generate this code.
/// - [`View::event`] is called when an event occurs. The should then handle the
///     event. Custom events can be send using [`EventCx::cmd`].
/// - [`View::layout`] is called when the view needs to be laid out. A leaf view
///     should compute it's own size in accordance with the given [`Space`], and
///     return it. A container view should pass an appropriate [`Space`] to it's
///     contents and the compute it's own size based on the contents' size(s).
/// - [`View::draw`] is called when the view needs to be drawn. See [`Canvas`]
///     for more information on drawing.
///
/// For examples see the implementation of views like [`Button`] or [`Checkbox`].
///
/// [`Content`]: crate::Content
/// [`ContentSequence`]: crate::ContentSequence
/// [`ViewState`]: crate::ViewState
/// [`Rebuild`]: crate::Rebuild
/// [`Button`]: crate::views::Button
/// [`Checkbox`]: crate::views::Checkbox
pub trait View<T> {
    /// The state of the view, see top-level documentation for more information.
    type State;

    /// Build the view state, see top-level documentation for more information.
    fn build(&mut self, cx: &mut BuildCx, data: &mut T) -> Self::State;

    /// Rebuild the view state, see top-level documentation for more information.
    fn rebuild(&mut self, state: &mut Self::State, cx: &mut RebuildCx, data: &mut T, old: &Self);

    /// Handle an event, see top-level documentation for more information.
    fn event(&mut self, state: &mut Self::State, cx: &mut EventCx, data: &mut T, event: &Event);

    /// Layout the view, see top-level documentation for more information.
    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut LayoutCx,
        data: &mut T,
        space: Space,
    ) -> Size;

    /// Draw the view, see top-level documentation for more information.
    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawCx, data: &mut T, canvas: &mut Canvas);
}

impl<T> View<T> for () {
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
        _cx: &mut DrawCx,
        _data: &mut T,
        _canvas: &mut Canvas,
    ) {
    }
}