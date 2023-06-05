/// A filter for an image.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ImageFilter {
    /// A linear filter (default).
    #[default]
    Linear,
    /// A nearest-pixel filter.
    Nearest,
}
