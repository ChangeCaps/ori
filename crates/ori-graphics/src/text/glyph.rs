use crate::Rect;

/// A laid out glyph.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Glyph {
    /// The character of the glyph.
    pub code: char,
    /// The rect of the glyph.
    pub rect: Rect,
    /// The byte offset of the glyph.
    pub byte_offset: usize,
    /// The line of the glyph.
    pub line: usize,
    /// The baseline of the glyph.
    pub baseline: f32,
    /// The decent of the glyph.
    pub line_descent: f32,
    /// The ascent of the glyph.
    pub line_ascent: f32,
    /// The advance of the glyph.
    pub advance: f32,
}
