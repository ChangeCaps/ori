//! A [`wgpu`] backend for Ori.
//!
//! See [`WgpuBackend`] for more information.

mod backend;
mod blit;
#[cfg(feature = "canvas")]
mod canvas;
mod image;
mod mesh;
mod quad;
mod renderer;

pub use backend::*;
pub use blit::*;
#[cfg(feature = "canvas")]
pub use canvas::*;
pub use image::*;
pub use mesh::*;
pub use quad::*;
pub use renderer::*;

pub mod prelude {
    //! A collection of commonly used types and traits.

    #[cfg(feature = "canvas")]
    pub use crate::canvas::WgpuCanvas;
}
