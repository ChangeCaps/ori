//! Ori is a framework for building cross-platform native gui applications.
//!
//! Ori uses a reactive, declarative programming model to build applications.
//! This means that the application is built by composing components together.
//!
//! # Example
//! ```no_run
//! # /* ------------------------------ IMPORTANT ------------------------------
//! #  * REMEMBER TO UPDATE 'examples/readme.rs' AND 'README.md' WHEN CHANGING THIS EXAMPLE
//! #  * ---------------------------------------------------------------------- */
//! use ori::prelude::*;
//!
//! // define the ui
//! fn ui(cx: Scope) -> impl View {
//!     // create a signal that will hold the state of the counter
//!     let counter = signal(cx, 0);
//!
//!     // we use the reactive! macro to create a reactive ui component
//!     let text = reactive!(format!("Clicked {} times", counter.get()));
//!
//!     // we create a button that increments the counter when pressed
//!     let button = Button::new(text).on_press(move |_| *counter.modify() += 1);
//!
//!     // we center the button in the window
//!     Align::center(button)
//! }
//!
//! fn main() {
//!     // configure and run the application
//!     App::new(ui).title("Readme (examples/readme.rs)").run();
//! }
//! ```

pub use ori_core as core;
pub use ori_graphics as graphics;
pub use ori_reactive as reactive;

#[cfg(feature = "font-awesome")]
pub use ori_font_awesome as font_awesome;

#[cfg(feature = "winit")]
pub use ori_winit as winit;

#[cfg(feature = "wgpu")]
pub use ori_wgpu as wgpu;

pub mod prelude {
    //! A collection of commonly used types and traits.

    pub use ori_core::prelude::*;
    pub use ori_graphics::prelude::*;
    pub use ori_reactive::prelude::*;

    #[cfg(feature = "font-awesome")]
    pub use ori_font_awesome::prelude::*;
    #[cfg(feature = "wgpu")]
    pub use ori_wgpu::prelude::*;
    #[cfg(feature = "winit")]
    pub use ori_winit::prelude::*;
}
