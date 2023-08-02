use glam::UVec2;
use ori_graphics::{Color, ImageData, ImageSource};

use crate::{Cursor, WindowId};

/// A descriptor for a window.
#[derive(Clone, Debug, PartialEq)]
pub struct Window {
    id: WindowId,
    /// The title of the window.
    pub title: String,
    /// Whether the window is resizable.
    pub resizable: bool,
    /// Whether the window has decorations.
    pub decorated: bool,
    /// The clear color of the window.
    pub clear_color: Color,
    /// The icon of the window.
    pub icon: Option<ImageData>,
    /// The scale of the window.
    pub scale: f32,
    /// The size of the window.
    pub size: UVec2,
    /// Whether the window is minimized.
    pub minimized: bool,
    /// Whether the window is maximized.
    pub maximized: bool,
    /// Whether the window is visible.
    pub visible: bool,
    /// The cursor of the window.
    pub cursor: Cursor,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            id: WindowId::new(),
            title: String::from("Ori App"),
            resizable: true,
            decorated: true,
            clear_color: Color::WHITE,
            icon: None,
            scale: 1.0,
            size: UVec2::new(800, 600),
            minimized: false,
            maximized: false,
            visible: true,
            cursor: Cursor::default(),
        }
    }
}

impl Window {
    /// Create a new window descriptor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the [`WindowId`] of the window.
    pub const fn id(&self) -> WindowId {
        self.id
    }
}

impl WindowBuilder for Window {
    fn window_mut(&mut self) -> &mut Window {
        self
    }
}

/// A trait for building a [`Window`].
pub trait WindowBuilder: Sized {
    fn window_mut(&mut self) -> &mut Window;

    /// Sets the `title` of the window.
    fn title(mut self, title: impl Into<String>) -> Self {
        self.window_mut().title = title.into();
        self
    }

    /// Sets whether the window is `resizable`.
    fn resizable(mut self, resizable: bool) -> Self {
        self.window_mut().resizable = resizable;
        self
    }

    /// Sets whether the window has `decorations`.
    fn decorated(mut self, decorated: bool) -> Self {
        self.window_mut().decorated = decorated;
        self
    }

    /// Sets the `size` of the window.
    fn size(mut self, width: u32, height: u32) -> Self {
        self.window_mut().size = UVec2::new(width, height);
        self
    }

    /// Sets the `width` of the window.
    fn width(mut self, width: u32) -> Self {
        self.window_mut().size.x = width;
        self
    }

    /// Sets the `height` of the window.
    fn height(mut self, height: u32) -> Self {
        self.window_mut().size.y = height;
        self
    }

    /// Sets the `maximized` state of the window.
    fn maximized(mut self, maximized: bool) -> Self {
        self.window_mut().maximized = maximized;
        self
    }

    /// Sets the `clear_color` of the window.
    fn clear_color(mut self, clear_color: Color) -> Self {
        self.window_mut().clear_color = clear_color;
        self
    }

    /// Sets the `clear_color` of the window to [`Color::TRANSPARENT`].
    fn transparent(mut self) -> Self {
        self.window_mut().clear_color = Color::TRANSPARENT;
        self
    }

    /// Sets the `icon` of the window.
    fn icon(mut self, icon: impl Into<ImageSource>) -> Self {
        self.window_mut().icon = Some(icon.into().load());
        self
    }

    /// Sets the `scale` of the window.
    fn scale(mut self, scale: f32) -> Self {
        self.window_mut().scale = scale;
        self
    }

    /// Sets the `visible` state of the window.
    fn visible(mut self, visible: bool) -> Self {
        self.window_mut().visible = visible;
        self
    }
}
