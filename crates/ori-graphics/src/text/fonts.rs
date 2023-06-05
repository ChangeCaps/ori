use std::{collections::HashMap, io, path::Path, sync::Arc};

use fontdue::{
    layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
    Font, FontSettings, Metrics,
};
use glam::Vec2;

use crate::{
    FontAtlas, FontQuery, Glyph, Mesh, Rect, Renderer, TextAlign, TextSection, TextWrap, Vertex,
};

/// An error that occurred while loading fonts.
#[derive(Debug)]
pub enum FontsError {
    /// An I/O error occurred.
    Io(io::Error),
    /// A fontdue error occurred.
    Fontdue(&'static str),
}

impl From<io::Error> for FontsError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<&'static str> for FontsError {
    fn from(err: &'static str) -> Self {
        Self::Fontdue(err)
    }
}

/// A collection of loaded fonts.
#[derive(Clone, Debug, Default)]
pub struct Fonts {
    db: fontdb::Database,
    font_cache: HashMap<fontdb::ID, Option<Arc<Font>>>,
    font_atlases: HashMap<fontdb::ID, FontAtlas>,
    query_cache: HashMap<FontQuery, fontdb::ID>,
}

impl Fonts {
    /// Creates a new font collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads a font from `data`.
    pub fn load_font_data(&mut self, data: Vec<u8>) {
        self.db.load_font_data(data);
    }

    /// Loads a font from a file.
    pub fn load_font_file(&mut self, path: impl AsRef<Path>) -> Result<(), FontsError> {
        self.db.load_font_file(path)?;
        Ok(())
    }

    /// Loads all fonts from a directory.
    pub fn load_fonts_dir(&mut self, path: impl AsRef<Path>) {
        self.db.load_fonts_dir(path);
    }

    /// Loads the system fonts.
    pub fn load_system_fonts(&mut self) {
        self.db.load_system_fonts();
        self.db.set_serif_family("Noto Serif");
        self.db.set_sans_serif_family("Noto Sans");
        self.db.set_monospace_family("Noto Sans Mono");
        self.db.set_cursive_family("Comic Sans MS");
        self.db.set_fantasy_family("Impact");
    }

    /// Queries the font collection for a font matching `query`.
    pub fn query_id(&mut self, query: &FontQuery) -> Option<fontdb::ID> {
        if let Some(id) = self.query_cache.get(query) {
            return Some(*id);
        }

        let fontdb_query = fontdb::Query {
            families: &[query.family.to_fontdb()],
            weight: query.weight.to_fontdb(),
            stretch: query.stretch.to_fontdb(),
            style: query.style.to_fontdb(),
        };

        let id = self.db.query(&fontdb_query)?;

        self.query_cache.insert(query.clone(), id);

        Some(id)
    }

    /// Queries the font collection for a font matching `query`.
    pub fn query(&mut self, query: &FontQuery) -> Option<Arc<Font>> {
        let id = self.query_id(query)?;
        self.get_font(id)
    }

    /// Gets a font from the font collection.
    pub fn get_font(&mut self, id: fontdb::ID) -> Option<Arc<Font>> {
        if let Some(font) = self.font_cache.get(&id) {
            return font.clone();
        }

        let font = self.db.with_face_data(id, |data, index| {
            let settings = FontSettings {
                scale: 80.0,
                collection_index: index,
            };

            Font::from_bytes(data, settings)
        });
        let font = Arc::new(font?.ok()?);

        self.font_cache.insert(id, Some(font.clone()));

        Some(font)
    }

    /// Queries the font collection for a font atlas matching `query`.
    pub fn query_atlas(&mut self, query: &FontQuery) -> Option<&mut FontAtlas> {
        let id = self.query_id(query)?;

        if self.font_atlases.contains_key(&id) {
            return self.font_atlases.get_mut(&id);
        }

        let atlas = FontAtlas::new();
        Some(self.font_atlases.entry(id).or_insert(atlas))
    }

    /// Gets a font atlas from the font collection.
    pub fn get_atlas(&mut self, id: fontdb::ID) -> &mut FontAtlas {
        self.font_atlases.entry(id).or_insert_with(FontAtlas::new)
    }

    fn text_layout_inner(&mut self, font: &Font, text: &TextSection<'_>) -> Option<Layout> {
        let max_width = match text.wrap {
            TextWrap::None => None,
            _ => Some(text.rect.width()),
        };

        let max_height = match text.wrap {
            TextWrap::None => Some(text.rect.height()),
            _ => None,
        };

        let settings = LayoutSettings {
            x: text.rect.min.x.round(),
            y: text.rect.min.y.round(),
            max_width,
            max_height,
            horizontal_align: text.h_align.to_horizontal(),
            vertical_align: text.v_align.to_vertical(),
            line_height: text.line_height,
            wrap_style: text.wrap.to_fontdue(),
            wrap_hard_breaks: true,
        };

        let text_style = TextStyle {
            text: text.text,
            px: text.font_size,
            font_index: 0,
            user_data: (),
        };

        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&settings);
        layout.append(&[font], &text_style);

        Some(layout)
    }

    /// Creates a text layout for `text`.
    pub fn text_layout(&mut self, text: &TextSection<'_>) -> Option<Layout> {
        let id = self.query_id(&text.font_query())?;
        let font = self.get_font(id)?;

        self.text_layout_inner(&font, text)
    }

    /// Creates a text layout for `text` and returns the glyphs.
    pub fn text_glyphs(&mut self, text: &TextSection<'_>) -> Option<Vec<Glyph>> {
        let id = self.query_id(&text.font_query())?;
        let font = self.get_font(id)?;

        let layout = self.text_layout_inner(&font, text)?;

        self.layout_glyphs(&font, &layout)
    }

    fn layout_glyphs(&self, font: &Font, layout: &Layout) -> Option<Vec<Glyph>> {
        if layout.glyphs().is_empty() {
            return None;
        }

        let mut glyphs = Vec::new();

        for (line_index, line) in layout.lines().into_iter().flatten().enumerate() {
            if line.glyph_end < line.glyph_start {
                continue;
            }

            for glyph in &layout.glyphs()[line.glyph_start..=line.glyph_end] {
                let metrics = if !glyph.char_data.is_control() {
                    font.metrics(glyph.parent, glyph.key.px)
                } else {
                    Metrics::default()
                };
                let advance = metrics.advance_width.ceil();

                let min = Vec2::new(glyph.x, glyph.y);
                let size = Vec2::new(metrics.width as f32, metrics.height as f32);

                let glyph = Glyph {
                    code: glyph.parent,
                    rect: Rect::min_size(min, size),
                    byte_offset: glyph.byte_offset,
                    line: line_index,
                    baseline: line.baseline_y,
                    line_descent: line.min_descent,
                    line_ascent: line.max_ascent,
                    advance,
                };

                glyphs.push(glyph);
            }
        }

        Some(glyphs)
    }

    fn measure_layout(&self, font: &Font, layout: &Layout) -> Option<Vec2> {
        if layout.glyphs().is_empty() {
            return None;
        }

        let mut width = 0.0;

        for line in layout.lines().into_iter().flatten() {
            let mut line_width = 0.0;

            if line.glyph_end < line.glyph_start {
                continue;
            }

            for glyph in &layout.glyphs()[line.glyph_start..=line.glyph_end] {
                let metrics = if !glyph.char_data.is_control() {
                    font.metrics(glyph.parent, glyph.key.px)
                } else {
                    Metrics::default()
                };
                let advance = metrics.advance_width.ceil();

                line_width += advance;
            }

            width = f32::max(width, line_width);
        }

        Some(Vec2::new(width, layout.height()))
    }

    /// Measures the size of `text`, and returns the smallest [`Rect`] that can contains it.
    pub fn measure_text(&mut self, text: &TextSection<'_>) -> Option<Rect> {
        let font = self.query(&text.font_query())?;
        let layout = self.text_layout_inner(&font, text)?;
        let size = self.measure_layout(&font, &layout)?;
        let rect = Rect::min_size(text.rect.min, size);
        Some(rect)
    }

    /// Creates a mesh for `text`.
    pub fn text_mesh(&mut self, renderer: &dyn Renderer, text: &TextSection<'_>) -> Option<Mesh> {
        let id = self.query_id(&text.font_query())?;
        let font = self.get_font(id)?;
        let layout = self.text_layout_inner(&font, text)?;
        let layout_size = self.measure_layout(&font, &layout)?;
        let atlas = self.get_atlas(id);

        let mut glyphs = Vec::<Rect>::new();

        'outer: loop {
            for glyph in layout.glyphs() {
                match atlas.glyph_rect_uv(renderer, &font, glyph.key) {
                    Some(rect) => glyphs.push(rect),
                    None => {
                        atlas.grow(renderer);
                        continue 'outer;
                    }
                }
            }

            break;
        }

        let x_offset = if text.wrap == TextWrap::None {
            match text.h_align {
                TextAlign::Left => 0.0,
                TextAlign::Center => (text.rect.width() - layout_size.x) / 2.0,
                TextAlign::Right => text.rect.width() - layout_size.x,
            }
        } else {
            0.0
        };

        let y_offset = if text.wrap != TextWrap::None {
            match text.v_align {
                TextAlign::Top => 0.0,
                TextAlign::Center => (text.rect.height() - layout_size.y) / 2.0,
                TextAlign::Bottom => text.rect.height() - layout_size.y,
            }
        } else {
            0.0
        };

        let offset = Vec2::new(x_offset, y_offset);

        let mut mesh = Mesh::new();

        for (glyph, uv) in layout.glyphs().iter().zip(glyphs) {
            let min = Vec2::new(glyph.x, glyph.y);
            let size = Vec2::new(glyph.width as f32, glyph.height as f32);
            let glyph_rect = Rect::min_size(min, size);

            let index = mesh.vertices.len() as u32;

            mesh.vertices.push(Vertex {
                position: glyph_rect.top_left() + offset,
                uv: uv.top_left(),
                color: text.color,
            });
            mesh.vertices.push(Vertex {
                position: glyph_rect.top_right() + offset,
                uv: uv.top_right(),
                color: text.color,
            });
            mesh.vertices.push(Vertex {
                position: glyph_rect.bottom_right() + offset,
                uv: uv.bottom_right(),
                color: text.color,
            });
            mesh.vertices.push(Vertex {
                position: glyph_rect.bottom_left() + offset,
                uv: uv.bottom_left(),
                color: text.color,
            });

            mesh.indices.push(index);
            mesh.indices.push(index + 1);
            mesh.indices.push(index + 2);
            mesh.indices.push(index);
            mesh.indices.push(index + 2);
            mesh.indices.push(index + 3);
        }

        mesh.image = atlas.image().cloned();

        Some(mesh)
    }
}
