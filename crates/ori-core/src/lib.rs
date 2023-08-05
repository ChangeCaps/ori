//! The core crate for the Ori UI framework.

mod build;
mod build_ui;
mod children;
mod context;
mod debug;
mod element;
mod elements;
mod event;
mod flex;
pub mod function;
mod layout;
mod metrics;
mod node;
mod ui;
mod view;
mod window;

pub use build::*;
pub use build_ui::*;
pub use children::*;
pub use context::*;
pub use debug::*;
pub use element::*;
pub use elements::*;
pub use event::*;
pub use flex::*;
pub use layout::*;
pub use metrics::*;
pub use node::*;
pub use ui::*;
pub use view::*;
pub use window::*;

pub use glam as math;
pub use tracing;

pub mod prelude {
    //! A collection of commonly used types and traits.

    pub use crate::build::Parent;
    pub use crate::children::Children;
    pub use crate::context::{Context, DrawContext, EventContext, LayoutContext};
    pub use crate::element::Element;
    pub use crate::elements::*;
    pub use crate::event::{
        CloseWindow, Cursor, DragWindow, Key, KeyboardEvent, Modifiers, OpenWindow, PointerButton,
        PointerEvent, RequestRedrawEvent, WindowClosedEvent, WindowResizedEvent,
    };
    pub use crate::flex::FlexLayout;
    pub use crate::function::*;
    pub use crate::layout::{
        AlignItem, AvailableSpace, Axis, FlexWrap, JustifyContent, Margin, Padding,
    };
    pub use crate::node::{Node, NodeRef};
    pub use crate::ui::UiBuilder;
    pub use crate::view::{IntoView, View};
    pub use crate::window::{Window, WindowBuilder, WindowId};

    pub use glam::*;
    pub use tracing::{debug, error, info, trace, warn};

    pub use ori_macro::{view, Build};
}
