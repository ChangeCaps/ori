use std::{ops::Deref, vec};

use fontdue::layout::GlyphRasterConfig;
use glam::Vec2;

use crate::{Color, Rect, TextAlign, TextWrap};

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
    /// The key of the glyph.
    pub key: GlyphRasterConfig,
}

#[derive(Clone, Debug, Default)]
pub struct Glyphs {
    pub(crate) glyphs: Vec<Glyph>,
    pub(crate) size: Vec2,
    pub(crate) font: fontdb::ID,
    pub(crate) wrap: TextWrap,
    pub(crate) h_align: TextAlign,
    pub(crate) v_align: TextAlign,
    pub(crate) color: Color,
}

impl Glyphs {
    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn font(&self) -> fontdb::ID {
        self.font
    }

    pub fn wrap(&self) -> TextWrap {
        self.wrap
    }

    pub fn h_align(&self) -> TextAlign {
        self.h_align
    }

    pub fn v_align(&self) -> TextAlign {
        self.v_align
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Deref for Glyphs {
    type Target = [Glyph];

    fn deref(&self) -> &Self::Target {
        &self.glyphs
    }
}

impl IntoIterator for Glyphs {
    type Item = Glyph;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.glyphs.into_iter()
    }
}

impl<'a> IntoIterator for &'a Glyphs {
    type Item = &'a Glyph;
    type IntoIter = std::slice::Iter<'a, Glyph>;

    fn into_iter(self) -> Self::IntoIter {
        self.glyphs.iter()
    }
}
