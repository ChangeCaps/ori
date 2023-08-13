mod icon;
mod kind;

pub use icon::*;
pub use kind::*;

pub mod prelude {
    pub use crate::{Icon, IconKind};
}
