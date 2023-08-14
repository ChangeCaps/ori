use std::{
    fmt::Debug,
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
#[derive(Default)]
pub struct Suspense {
    future: Mutex<Option<SuspenseFuture>>,
    receiver: Option<Receiver<Node>>,
    content: Node,
}

impl Debug for Suspense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Suspense")
            .field("receiver", &self.receiver)
            .field("content", &self.content)
            .finish()
    }
}

impl Suspense {
    /// Create a new suspense view.
    pub fn new(content: impl Future<Output = impl Into<Node>> + Send + 'static) -> Self {
        Self {
            future: Mutex::new(Some(Box::pin(async { content.await.into() }))),
            receiver: None,
            content: Default::default(),
        }
    }

    /// Set the fallback content to display while the future is running.
    pub fn fallback(mut self, fallback: impl Into<Node>) -> Self {
        self.content = fallback.into();
        self
    }

    fn spawn_content(&mut self, cx: &mut Context<'_>) {
        if let Some(content) = self.future.lock().take() {
            let (tx, rx) = channel();
            let event_sink = cx.event_sink.clone();
            self.receiver = Some(rx);

            cx.spawn_future(async move {
                let content = content.await;
                let _ = tx.send(content);
                event_sink.send(RequestLayoutEvent);
                println!("done");
            });
        }
    }

    fn recv(&mut self) {
        if let Some(ref mut rx) = self.receiver {
            if let Ok(content) = rx.try_recv() {
                self.content = content;
                self.receiver = None;
            }
        }
    }

    fn prepare(&mut self, cx: &mut Context<'_>) {
        self.spawn_content(cx);
        self.recv();
    }
}

impl View for Suspense {
    fn event(&mut self, cx: &mut EventContext<'_>, event: &Event) {
        self.prepare(cx);
        self.content.event(cx, event);
    }

    fn layout(&mut self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        self.prepare(cx);
        self.content.layout(cx, space)
    }

    fn draw(&mut self, cx: &mut DrawContext<'_>) {
        self.prepare(cx);
        self.content.draw(cx);
    }
}
