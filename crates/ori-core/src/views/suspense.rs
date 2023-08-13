use std::{
    future::Future,
    pin::Pin,
    sync::mpsc::{channel, Receiver},
};

use ori_graphics::math::Vec2;
use ori_reactive::Event;
use parking_lot::Mutex;

use crate::{
    AvailableSpace, Context, DrawContext, EventContext, LayoutContext, Node, RequestLayoutEvent,
    View,
};

type SuspenseFuture = Pin<Box<dyn Future<Output = Node> + Send>>;

/// A view that can display asynchronously created content.
///
/// The content is created by a future that is spawned when the view is first
/// drawn. While the future is running, the view displays a fallback.
pub struct Suspense {
    future: Mutex<Option<SuspenseFuture>>,
    receiver: Option<Receiver<Node>>,
    content: Node,
}

impl Suspense {
    pub fn new(content: impl Future<Output = Node> + Send + 'static) -> Self {
        Self {
            future: Mutex::new(Some(Box::pin(content))),
            receiver: None,
            content: Default::default(),
        }
    }

    pub fn fallback(mut self, fallback: impl Into<Node>) -> Self {
        self.content = fallback.into();
        self
    }

    fn take_content(&self) -> Option<SuspenseFuture> {
        self.future.lock().take()
    }

    fn spawn_content(&mut self, cx: &mut Context<'_>) {
        if let Some(content) = self.take_content() {
            let (tx, rx) = channel();
            let event_sink = cx.event_sink.clone();
            self.receiver = Some(rx);

            cx.spawn_future(async move {
                let content = content.await;
                let _ = tx.send(content);
                event_sink.send(RequestLayoutEvent);
            });
        }
    }

    fn recv(&mut self) {
        if let Some(rx) = self.receiver.take() {
            if let Ok(content) = rx.try_recv() {
                self.content = content;
            }
        }
    }
}

impl View for Suspense {
    fn event(&mut self, cx: &mut EventContext<'_>, event: &Event) {
        self.spawn_content(cx);
        self.recv();
        self.content.event(cx, event);
    }

    fn layout(&mut self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        self.spawn_content(cx);
        self.recv();
        self.content.layout(cx, space)
    }

    fn draw(&mut self, cx: &mut DrawContext<'_>) {
        self.spawn_content(cx);
        self.recv();
        self.content.draw(cx);
    }
}
