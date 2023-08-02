use ori_reactive::{Scope, Signal};

use crate::{BuildUi, CloseWindow, DragWindow, OpenWindow, Window, WindowId};

/// Extension trait for [`Scope`] that provides window related methods.
pub trait ScopeWindowExt {
    /// Returns a signal that with the current window.
    fn window(self) -> Signal<Window>;

    /// Open a new window, see [`OpenWindow`] for more information.
    fn open_window<I>(self, window: Window, ui: impl BuildUi<I>) -> WindowId;

    /// Close the current window, see [`CloseWindow`] for more information.
    fn close_window(self);

    /// Drag the current window, see [`DragWindow`] for more information.
    fn drag_window(self);
}

impl ScopeWindowExt for Scope {
    fn window(self) -> Signal<Window> {
        self.context::<Signal<Window>>()
    }

    fn open_window<I>(self, window: Window, ui: impl BuildUi<I>) -> WindowId {
        let id = window.id();
        self.emit(OpenWindow::new(window, ui));
        id
    }

    fn close_window(self) {
        self.emit(CloseWindow::new());
    }

    fn drag_window(self) {
        self.emit(DragWindow::new());
    }
}
