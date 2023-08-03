use ori_reactive::{Modify, Scope, Signal};

use crate::{
    BuildUi, CloseWindow, DragWindow, IntoView, NodeRef, OpenWindow, View, Window, WindowId,
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
    cx.emit(DragWindow::new());
}

/// Creates a new dynamic [`View`] from a Ui function.
pub fn dynamic<I: IntoView>(cx: Scope, mut f: impl BuildUi<I>) -> View {
    View::dynamic(cx.owned_memo_scoped(move |cx| f.ui(cx)))
}

/// Creates a new [`NodeRef`].
pub fn node_ref(cx: Scope) -> NodeRef {
    NodeRef::new(cx)
}
