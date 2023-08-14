//! The core crate for the Ori UI framework.

mod border;
mod context;
mod draw;
mod event;
pub mod function;
mod layout;
mod metrics;
mod node;
mod reactive;
mod root;
mod state;
mod theme;
mod tree;
mod ui;
mod view;
pub mod views;
mod window;

pub use border::*;
pub use context::*;
pub use draw::*;
pub use event::*;
pub use layout::*;
pub use metrics::*;
pub use node::*;
pub use reactive::*;
pub use state::*;
pub use theme::*;
pub use tree::*;
pub use ui::*;
pub use view::*;
pub use window::*;

pub use ori_graphics::math;
pub use tracing;

pub mod prelude {
    //! A collection of commonly used types and traits.

    pub use crate::context::Context;
    pub use crate::event::{
        CloseWindow, Code, Cursor, DragWindow, KeyboardEvent, Modifiers, OpenWindow, PointerButton,
        PointerEvent, RequestRedrawEvent, WindowClosedEvent, WindowResizedEvent,
    };
    pub use crate::function::*;
    pub use crate::layout::*;
    pub use crate::math::*;
    pub use crate::node::Node;
    pub use crate::theme::{Key, Palette, Style, Styled, Theme};
    pub use crate::ui::UiBuilder;
    pub use crate::view::{DrawContext, EventContext, LayoutContext, View};
    pub use crate::views::*;
    pub use crate::window::{Window, WindowBuilder, WindowId};
    pub use crate::{hstack, vstack};

    pub use ori_macro::reactive;

    pub use tracing::{debug, error, info, trace, warn};
}
