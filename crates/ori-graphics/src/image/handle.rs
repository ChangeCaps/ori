use std::{
    any::Any,
    sync::{Arc, Weak},
};

use glam::Vec2;

use crate::ImageFilter;

/// A handle to a loaded image.
#[derive(Clone, Debug)]
pub struct ImageHandle {
    width: u32,
    height: u32,
    filter: ImageFilter,
    handle: Arc<dyn Any + Send + Sync>,
}

impl PartialEq for ImageHandle {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            Arc::as_ptr(&self.handle) as *const u8,
            Arc::as_ptr(&other.handle) as *const u8,
        )
    }
}

impl ImageHandle {
    /// Creates a new image handle. This is called by [`Renderer::create_image`](crate::Renderer::create_image)
    /// and should usually not be called manually.
    pub fn new<T: Any + Send + Sync>(
        handle: T,
        width: u32,
        height: u32,
        filter: ImageFilter,
    ) -> Self {
        Self {
            width,
            height,
            filter,
            handle: Arc::new(handle),
        }
    }

    pub fn from_arc<T: Any + Send + Sync>(
        handle: Arc<T>,
        width: u32,
        height: u32,
        filter: ImageFilter,
    ) -> Self {
        Self {
            width,
            height,
            filter,
            handle,
        }
    }

    /// Downgrades the image handle to a [`WeakImageHandle`].
    pub fn downgrade(&self) -> WeakImageHandle {
        WeakImageHandle {
            width: self.width,
            height: self.height,
            filter: self.filter,
            handle: Arc::downgrade(&self.handle),
        }
    }

    /// Tries to downcast the image handle to a concrete type.
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.handle.downcast_ref()
    }

    /// Tries to downcast the image handle to a concrete type.
    pub fn downcast_arc<T: Any + Send + Sync>(self) -> Option<Arc<T>> {
        Arc::downcast(self.handle).ok()
    }

    /// Returns the width of the image.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the filter of the image.
    pub fn filter(&self) -> ImageFilter {
        self.filter
    }

    /// Returns the size of the image.
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

/// A weak handle to a loaded image, see [`ImageHandle::downgrade`].
#[derive(Clone, Debug)]
pub struct WeakImageHandle {
    width: u32,
    height: u32,
    filter: ImageFilter,
    handle: Weak<dyn Any + Send + Sync>,
}

impl PartialEq for WeakImageHandle {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.handle, &other.handle)
    }
}

impl WeakImageHandle {
    /// Upgrades the image handle to an [`ImageHandle`].
    pub fn upgrade(&self) -> Option<ImageHandle> {
        Some(ImageHandle {
            width: self.width,
            height: self.height,
            filter: self.filter,
            handle: self.handle.upgrade()?,
        })
    }

    /// Returns true if the image is still alive, and can be upgraded.
    pub fn is_alive(&self) -> bool {
        self.handle.strong_count() > 0
    }

    /// Returns the filter of the image.
    pub fn filter(&self) -> ImageFilter {
        self.filter
    }

    /// Returns the width of the image.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the size of the image.
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}
