use std::fmt::Debug;

use ori_graphics::{math::UVec2, ImageData};
use ori_reactive::EventSink;

use crate::{Cursor, Window, WindowId};

/// A trait that defines the interface for a windowing backend.
///
/// All setter and getter functions assume that a window with the given `id` exists, and are allowed
/// to panic otherwise.
pub trait WindowBackend {
    /// The target on which the window is created. In the winit backend, this is the
    /// `EventLoopWindowTarget`.
    type Target<'a>;
    /// The surface used by the [`RenderBackend`](ori_graphics::RenderBackend), on which to create
    /// the renderer. In the wgpu backend, this is a tuple of the `RawDisplayHandle` and
    /// `RawWindowHandle`.
    type Surface;
    /// The error type returned by the backend.
    type Error: Debug;

    /// Creates a window on the backend.
    ///
    /// All functions below assume that `window.id()` is a valid for this backend.
    fn create_window(
        &mut self,
        target: Self::Target<'_>,
        window: &Window,
    ) -> Result<(), Self::Error>;

    /// Creates a surface for a window with `id`.
    fn create_surface(&self, id: WindowId) -> Result<Self::Surface, Self::Error>;

    /// Creates an event sink for a window with `id`.
    fn create_event_sink(&self, id: WindowId) -> Result<EventSink, Self::Error>;

    /// Closes a window with `id`.
    fn close_window(&mut self, id: WindowId);

    /// Requests a redraw for a window with `id`.
    fn request_redraw(&mut self, id: WindowId);

    /// Begins a drag operation for a window with `id`.
    fn drag_window(&mut self, id: WindowId);

    /// Returns the title of a window with `id`.
    fn get_title(&self, id: WindowId) -> String;

    /// Sets the `title` of a window with `id`.
    fn set_title(&mut self, id: WindowId, title: impl Into<String>);

    /// Returns whether a window with `id` is resizable.
    fn get_resizable(&self, id: WindowId) -> bool;

    /// Sets whether a window with `id` is `resizable`.
    fn set_resizable(&mut self, id: WindowId, resizable: bool);

    /// Returns whether a window with `id` has decorations.
    fn get_decorations(&self, id: WindowId) -> bool;

    /// Sets whether a window with `id` has `decorations`.
    fn set_decorations(&mut self, id: WindowId, decorations: bool);

    /// Sets whether a window with `id` is `transparent`.
    fn set_transparent(&mut self, id: WindowId, transparent: bool);

    /// Sets the `icon` of a window with `id`.
    fn set_icon(&mut self, id: WindowId, icon: Option<ImageData>);

    /// Returns the position of a window with `id`.
    fn get_size(&self, id: WindowId) -> UVec2;

    /// Sets the `size` of a window with `id`.
    fn set_size(&mut self, id: WindowId, size: UVec2);

    /// Returns whether a window with `id` is minimized.
    fn get_minimized(&self, id: WindowId) -> bool;

    /// Sets whether a window with `id` is `minimized`.
    fn set_minimized(&mut self, id: WindowId, minimized: bool);

    /// Returns whether a window with `id` is maximized.
    fn get_maximized(&self, id: WindowId) -> bool;

    /// Sets whether a window with `id` is `maximized`.
    fn set_maximized(&mut self, id: WindowId, maximized: bool);

    /// Returns whether a window with `id` is visible.
    fn get_visible(&self, id: WindowId) -> bool;

    /// Sets whether a window with `id` is `visible`.
    fn set_visible(&mut self, id: WindowId, visible: bool);

    /// Sets the `cursor` of a window with `id`.
    fn set_cursor(&mut self, id: WindowId, cursor: Cursor);
}
