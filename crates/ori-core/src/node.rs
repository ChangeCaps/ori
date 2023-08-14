use std::{any::Any, fmt::Debug};

use ori_graphics::{math::Vec2, Affine};
use ori_reactive::Event;

use crate::{AvailableSpace, DrawContext, EventContext, LayoutContext, Padding, View};

impl<T: View> From<T> for Node {
    fn from(view: T) -> Self {
        Self {
            state: ViewState::default(),
            view: Box::new(view),
        }
    }
}

impl<T: View, U: Debug> From<Result<T, U>> for Node {
    fn from(value: Result<T, U>) -> Self {
        match value {
            Ok(value) => Node::new(value),
            Err(err) => Node::new(format!("{:?}", err)),
        }
    }
}

#[derive(Debug, Default)]
pub struct ViewState {
    /* flags */
    pub(crate) hovered: bool,
    pub(crate) active: bool,
    pub(crate) has_active: bool,
    /* layout */
    pub(crate) size: Vec2,
    pub(crate) transform: Affine,
    /* state */
    pub(crate) state: Option<Box<dyn Any + Send>>,
}

impl ViewState {
    pub(crate) fn take_state(&mut self) -> Option<Box<dyn Any + Send>> {
        self.state.take()
    }

    pub(crate) fn set_state(&mut self, state: Box<dyn Any + Send>) {
        self.state = Some(state);
    }

    pub(crate) fn propagate(&mut self, child: &mut ViewState) {
        child.has_active = child.active;
        self.has_active |= child.has_active;
    }

    /// Set the transform of the view.
    pub fn set_transform(&mut self, transform: Affine) {
        self.transform = transform;
    }

    /// Get the transform of the view.
    pub fn transform(&self) -> Affine {
        self.transform
    }

    /// Set whether the view is active.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Set whether the view is hovered.
    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    /// Get whether the view is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get whether the view is hovered.
    pub fn is_hovered(&self) -> bool {
        self.hovered
    }
}

pub struct Node {
    state: ViewState,
    view: Box<dyn View>,
}

impl Default for Node {
    fn default() -> Self {
        Self::empty()
    }
}

impl Node {
    pub fn new(view: impl Into<Node>) -> Self {
        view.into()
    }

    pub fn empty() -> Self {
        Self {
            state: ViewState::default(),
            view: Box::new(()),
        }
    }

    pub fn view(&self) -> &dyn View {
        self.view.as_ref()
    }

    pub fn downcast_ref<T: View>(&self) -> Option<&T> {
        self.view.downcast_ref()
    }

    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    pub fn is_hovered(&self) -> bool {
        self.state.is_hovered()
    }

    pub fn transform(&self) -> Affine {
        self.state.transform()
    }

    pub fn set_transform(&mut self, transform: Affine) {
        self.state.set_transform(transform);
    }

    pub fn layout_padded(
        &mut self,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
        padding: Padding,
    ) -> Vec2 {
        let transform = Affine::translate(padding.offset(cx));
        self.state.set_transform(transform);

        let size = padding.size(cx);
        self.layout(cx, space.shrink(size)) + size
    }

    pub fn event(&mut self, cx: &mut EventContext<'_>, event: &Event) {
        cx.with_transform(self.state.transform, |cx| {
            let mut event_cx = EventContext::new(cx.context.borrow(), cx.transform);
            event_cx.view_state = &mut self.state;

            self.view.event(&mut event_cx, event);

            self.state.propagate(cx.view_state);
        });
    }

    pub fn layout(&mut self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        let mut layout_cx = LayoutContext::new(cx.context.borrow());
        layout_cx.view_state = &mut self.state;

        let size = self.view.layout(&mut layout_cx, space);
        self.state.size = size;

        self.state.propagate(cx.view_state);

        size
    }

    pub fn draw(&mut self, cx: &mut DrawContext<'_>) {
        cx.with_layer(1.0, |cx| {
            cx.with_transform(self.state.transform, |cx| {
                let mut draw_cx = DrawContext::new(cx.context.borrow(), cx.frame);
                draw_cx.view_state = &mut self.state;

                self.view.draw(&mut draw_cx);

                self.state.propagate(cx.view_state);
            });
        });
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node").finish()
    }
}
