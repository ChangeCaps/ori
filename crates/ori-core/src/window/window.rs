use std::{
    fmt::Display,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
    canvas::Color,
    event::{Pointer, PointerId},
    image::Image,
    layout::{Point, Size},
    view::ViewId,
};

use super::{Cursor, RawWindow};

/// A unique identifier for a window.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WindowId {
    index: usize,
}

impl Default for WindowId {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowId {
    /// Create a new [`WindowId`].
    pub fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let index = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        Self { index }
    }
}

impl Display for WindowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:x}", self.index)
    }
}

/// A handle to a window.
#[derive(Debug)]
pub struct Window {
    id: WindowId,
    raw: Box<dyn RawWindow>,
    pointers: Vec<Pointer>,
}

impl Window {
    /// Create a new window with the given raw window implementation.
    pub fn new(raw: Box<dyn RawWindow>, id: WindowId) -> Self {
        Self {
            id,
            raw,
            pointers: Vec::new(),
        }
    }

    /// Get the [`WindowId`].
    pub fn id(&self) -> WindowId {
        self.id
    }

    /// Get the pointers.
    pub fn pointers(&self) -> &[Pointer] {
        &self.pointers
    }

    /// Get the pointer with `id`.
    pub fn pointer(&self, id: PointerId) -> Option<&Pointer> {
        self.pointers.iter().find(|p| p.id() == id)
    }

    pub(crate) fn pointer_mut(&mut self, id: PointerId) -> Option<&mut Pointer> {
        self.pointers.iter_mut().find(|p| p.id() == id)
    }

    pub(crate) fn pointer_moved(&mut self, id: PointerId, position: Point) {
        if let Some(pointer) = self.pointer_mut(id) {
            pointer.position = position;
        } else {
            self.pointers.push(Pointer::new(id, position));
        }
    }

    pub(crate) fn pointer_left(&mut self, id: PointerId) {
        if let Some(index) = self.pointers.iter().position(|p| p.id() == id) {
            self.pointers.swap_remove(index);
        }
    }

    pub(crate) fn pointer_hovered(&mut self, id: PointerId, hovered: Option<ViewId>) {
        if let Some(pointer) = self.pointer_mut(id) {
            pointer.hovered = hovered;
        }
    }

    /// Get the pointer that is currently hovered over the given view.
    pub fn is_hovered(&self, view: ViewId) -> bool {
        self.pointers.iter().any(|p| p.hovered() == Some(view))
    }

    /// Try to downcast the window to a specific type.
    pub fn downcast_raw<T: RawWindow>(&self) -> Option<&T> {
        self.raw.downcast_ref()
    }

    /// Get the title of the window.
    pub fn title(&self) -> String {
        self.raw.title()
    }

    /// Set the title of the window.
    pub fn set_title(&mut self, title: &str) {
        self.raw.set_title(title);
    }

    /// Set the icon of the window.
    pub fn set_icon(&mut self, icon: Option<&Image>) {
        self.raw.set_icon(icon);
    }

    /// Get the size of the window.
    pub fn size(&self) -> Size {
        let (width, height) = self.raw.size();
        Size::new(width as f32, height as f32)
    }

    /// Get the physical size of the window.
    pub fn physical_size(&self) -> Size {
        self.size() * self.scale_factor()
    }

    /// Get the width of the window.
    pub fn width(&self) -> u32 {
        self.raw.size().0
    }

    /// Get the height of the window.
    pub fn height(&self) -> u32 {
        self.raw.size().1
    }

    /// Get the physical width of the window.
    pub fn physical_width(&self) -> u32 {
        let width = self.width() as f32 * self.scale_factor();
        width as u32
    }

    /// Get the physical height of the window.
    pub fn physical_height(&self) -> u32 {
        let height = self.height() as f32 * self.scale_factor();
        height as u32
    }

    /// Set the size of the window.
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.raw.set_size(width, height);
    }

    /// Get whether the window is resizable.
    pub fn resizable(&self) -> bool {
        self.raw.resizable()
    }

    /// Set whether the window is resizable.
    pub fn set_resizable(&mut self, resizable: bool) {
        self.raw.set_resizable(resizable);
    }

    /// Get whether the window is decorated.
    pub fn decorated(&self) -> bool {
        self.raw.decorated()
    }

    /// Set whether the window is decorated.
    pub fn set_decorated(&mut self, decorated: bool) {
        self.raw.set_decorated(decorated);
    }

    /// Get the scale factor of the window.
    pub fn scale_factor(&self) -> f32 {
        self.raw.scale_factor()
    }

    /// Get whether the window is minimized.
    pub fn minimized(&self) -> bool {
        self.raw.minimized()
    }

    /// Set whether the window is minimized.
    pub fn set_minimized(&mut self, minimized: bool) {
        self.raw.set_minimized(minimized);
    }

    /// Get whether the window is maximized.
    pub fn maximized(&self) -> bool {
        self.raw.maximized()
    }

    /// Set whether the window is maximized.
    pub fn set_maximized(&mut self, maximized: bool) {
        self.raw.set_maximized(maximized);
    }

    /// Get whether the window is visible.
    pub fn visible(&self) -> bool {
        self.raw.visible()
    }

    /// Set whether the window is visible.
    pub fn set_visible(&mut self, visible: bool) {
        self.raw.set_visible(visible);
    }

    /// Set the cursor of the window.
    pub fn set_cursor(&mut self, cursor: Cursor) {
        self.raw.set_cursor(cursor);
    }

    /// Get the background of the window.
    pub fn color(&self) -> Option<Color> {
        self.raw.color()
    }

    /// Set the background of the window.
    pub fn set_color(&mut self, color: Option<Color>) {
        self.raw.set_color(color);
    }

    /// Get whether the soft input is enabled.
    pub fn set_soft_input(&mut self, enabled: bool) {
        self.raw.set_soft_input(enabled);
    }

    /// Drag the window.
    pub fn drag(&mut self) {
        self.raw.drag();
    }

    /// Request a redraw of the window.
    pub fn request_draw(&mut self) {
        self.raw.request_draw();
    }
}
