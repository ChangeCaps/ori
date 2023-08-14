//! All-purpose widgets that can be used in any application.

mod align;
mod button;
mod check_box;
mod container;
mod image;
mod pad;
mod placeholder;
mod radio;
mod stack;
mod suspense;
mod text;
mod text_input;
mod themed;

pub use align::*;
pub use button::*;
pub use check_box::*;
pub use container::*;
pub use image::*;
pub use pad::*;
pub use placeholder::*;
pub use radio::*;
pub use stack::*;
pub use suspense::*;
pub use text::*;
pub use text_input::*;
pub use themed::*;

type EventCallback<T> = Box<dyn FnMut(&T) + Send>;
