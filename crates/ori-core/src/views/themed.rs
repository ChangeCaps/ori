use std::sync::Arc;

use ori_graphics::math::Vec2;
use ori_reactive::Event;

use crate::{AvailableSpace, DrawContext, EventContext, LayoutContext, Node, Theme, View};

pub struct Themed {
    pub content: Node,
    pub provider: Arc<dyn Fn(&mut Theme) + Send + Sync>,
}

impl Themed {
    pub fn new(
        content: impl Into<Node>,
        provider: impl Fn(&mut Theme) + Send + Sync + 'static,
    ) -> Self {
        Self {
            content: content.into(),
            provider: Arc::new(provider),
        }
    }
}

impl View for Themed {
    fn event(&mut self, cx: &mut EventContext<'_>, event: &Event) {
        let mut theme = cx.theme.clone();
        (self.provider)(&mut theme);

        let mut cx = cx.borrow();
        cx.theme = &theme;

        self.content.event(&mut cx, event);
    }

    fn layout(&mut self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        let mut theme = cx.theme.clone();
        (self.provider)(&mut theme);

        let mut cx = cx.borrow();
        cx.theme = &theme;

        self.content.layout(&mut cx, space)
    }

    fn draw(&mut self, cx: &mut DrawContext<'_>) {
        let mut theme = cx.theme.clone();
        (self.provider)(&mut theme);

        let mut cx = cx.borrow();
        cx.theme = &theme;

        self.content.draw(&mut cx);
    }
}
