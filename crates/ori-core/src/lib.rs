//! The core crate for the Ori UI framework.

mod build;
mod build_ui;
mod children;
mod context;
mod debug;
mod element;
mod elements;
mod event;
mod layout;
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
pub use layout::*;
pub use node::*;
pub use ui::*;
pub use view::*;
pub use window::*;

pub use glam as math;

pub mod prelude {
    //! A collection of commonly used types and traits.

    pub use crate::build::Parent;
    pub use crate::children::{Children, FlexLayout};
    pub use crate::context::{Context, DrawContext, EventContext, LayoutContext};
    pub use crate::element::Element;
    pub use crate::elements::*;
    pub use crate::event::{
        CloseWindow, Cursor, DragWindow, Key, KeyboardEvent, Modifiers, OpenWindow, PointerButton,
        PointerEvent, RequestRedrawEvent, WindowClosedEvent, WindowResizedEvent,
    };
    pub use crate::layout::{AlignItem, AvailableSpace, Axis, JustifyContent, Margin, Padding};
    pub use crate::node::{Node, NodeRef, ScopeNodeRefExt};
    pub use crate::view::{IntoView, ScopeViewExt, View};
    pub use crate::window::{ScopeWindowExt, Window, WindowBuilder, WindowId};

    pub use glam::*;
    pub use tracing::{debug, error, info, trace, warn};

    pub use ori_macro::{view, Build};
}
