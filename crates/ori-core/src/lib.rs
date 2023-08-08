//! The core crate for the Ori UI framework.

mod context;
mod dynamic;
mod event;
pub mod function;
mod layout;
mod metrics;
mod node;
mod state;
mod tree;
mod ui;
mod view;
pub mod views;
mod window;

pub use context::*;
pub use dynamic::*;
pub use event::*;
pub use layout::*;
pub use metrics::*;
pub use node::*;
pub use state::*;
pub use tree::*;
pub use ui::*;
pub use view::*;
pub use window::*;

pub use glam as math;
pub use tracing;

pub mod prelude {
    //! A collection of commonly used types and traits.

    pub use crate::context::Context;
    pub use crate::event::{
        CloseWindow, Cursor, DragWindow, Key, KeyboardEvent, Modifiers, OpenWindow, PointerButton,
        PointerEvent, RequestRedrawEvent, WindowClosedEvent, WindowResizedEvent,
    };
    pub use crate::function::*;
    pub use crate::layout::{AvailableSpace, Padding};
    pub use crate::node::Node;
    pub use crate::ui::UiBuilder;
    pub use crate::view::{DrawContext, EventContext, IntoView, LayoutContext, View};
    pub use crate::views::*;
    pub use crate::window::{Window, WindowBuilder, WindowId};
    pub use crate::{column, row};

    pub use glam::*;
    pub use tracing::{debug, error, info, trace, warn};
}
