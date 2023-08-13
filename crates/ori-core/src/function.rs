use std::sync::mpsc::channel;

use ori_reactive::{
    effect::current_effect, function::effect_scoped, prelude::emit, Modify, Scope, Signal,
};

use crate::{
    BuildUi, CloseWindow, DragWindow, OpenWindow, ReactiveNode, RequestAnimationFrame,
    RequestLayoutEvent, RequestRedrawEvent, Window, WindowId,
};

/// Returns a signal with the current window.
pub fn window(cx: Scope) -> Signal<Window> {
    cx.context()
}

/// Returns a modify with the current window.
pub fn window_mut(cx: Scope) -> Modify<Window> {
    window(cx).modify()
}

/// Open a new window, see [`OpenWindow`] for more information.
pub fn open_window<I>(cx: Scope, window: Window, ui: impl BuildUi<I>) -> WindowId {
    let id = window.id();
    cx.emit(OpenWindow::new(window, ui));
    id
}

/// Close the current window, see [`CloseWindow`] for more information.
pub fn close_window(cx: Scope) {
    cx.emit(CloseWindow::new());
}

/// Minimize the current window.
///
/// This will also unminimize the window, if it's already minimized.
pub fn minimize_window(cx: Scope) {
    window(cx).modify().minimize();
}

/// Maximize the current window.
///
/// This will also unmaximize the window, if it's already maximized.
pub fn maximize_window(cx: Scope) {
    window(cx).modify().maximize();
}

/// Drag the current window, see [`DragWindow`] for more information.
pub fn drag_window(cx: Scope) {
    emit(cx, DragWindow::new());
}

/// Request a redraw of the current window.
pub fn request_redraw(cx: Scope) {
    emit(cx, RequestRedrawEvent);
}

/// Request a layout of the current window.
pub fn request_layout(cx: Scope) {
    emit(cx, RequestLayoutEvent);
}

/// Request an animation frame.
///
/// This will re-evaluate the current effect just before the next draw.
pub fn request_animation_frame(cx: Scope) {
    if let Some(callback) = current_effect() {
        let request = RequestAnimationFrame::new(callback);
        emit(cx, request);
        request_redraw(cx);
    } else {
        tracing::warn!("animation frame requested outside effect");
    }
}

/// Creates a new reactive [`View`](crate::View) from a Ui function.
pub fn reactive<V>(cx: Scope, mut f: impl BuildUi<V>) -> ReactiveNode {
    let (tx, rx) = channel();

    effect_scoped(cx, move |cx| {
        let node = f.build(cx);
        let _ = tx.send(node);
        request_layout(cx);
    });

    ReactiveNode::new(rx)
}
