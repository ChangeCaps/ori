use std::{collections::HashMap, io, sync::Arc};

use cosmic_text::{
    fontdb::Source, Buffer, CacheKey, FontSystem, LayoutGlyph, SwashCache, SwashContent,
};

const ROBOTO_BLACK: &[u8] = include_bytes!("../../font/Roboto-Black.ttf");
const ROBOTO_BLACK_ITALIC: &[u8] = include_bytes!("../../font/Roboto-BlackItalic.ttf");
const ROBOTO_BOLD: &[u8] = include_bytes!("../../font/Roboto-Bold.ttf");
const ROBOTO_BOLD_ITALIC: &[u8] = include_bytes!("../../font/Roboto-BoldItalic.ttf");
const ROBOTO_ITALIC: &[u8] = include_bytes!("../../font/Roboto-Italic.ttf");
const ROBOTO_LIGHT: &[u8] = include_bytes!("../../font/Roboto-Light.ttf");
const ROBOTO_LIGHT_ITALIC: &[u8] = include_bytes!("../../font/Roboto-LightItalic.ttf");
const ROBOTO_MEDIUM: &[u8] = include_bytes!("../../font/Roboto-Medium.ttf");
const ROBOTO_MEDIUM_ITALIC: &[u8] = include_bytes!("../../font/Roboto-MediumItalic.ttf");
const ROBOTO_REGULAR: &[u8] = include_bytes!("../../font/Roboto-Regular.ttf");
const ROBOTO_THIN: &[u8] = include_bytes!("../../font/Roboto-Thin.ttf");
const ROBOTO_THIN_ITALIC: &[u8] = include_bytes!("../../font/Roboto-ThinItalic.ttf");

const ROBOTO_MONO: &[u8] = include_bytes!("../../font/RobotoMono.ttf");
const ROBOTO_MONO_ITALIC: &[u8] = include_bytes!("../../font/RobotoMono-Italic.ttf");

const EMBEDDED_FONTS: &[&[u8]] = &[
    ROBOTO_BLACK,
    ROBOTO_BLACK_ITALIC,
    ROBOTO_BOLD,
    ROBOTO_BOLD_ITALIC,
    ROBOTO_ITALIC,
    ROBOTO_LIGHT,
    ROBOTO_LIGHT_ITALIC,
    ROBOTO_MEDIUM,
    ROBOTO_MEDIUM_ITALIC,
    ROBOTO_REGULAR,
    ROBOTO_THIN,
    ROBOTO_THIN_ITALIC,
    ROBOTO_MONO,
    ROBOTO_MONO_ITALIC,
];

use crate::{
    layout::{Point, Size, Vector},
    prelude::{Canvas, Color, Image},
};

use super::FontSource;

/// A cached glyph.
#[derive(Clone, Debug)]
pub struct CachedGlyph {
    /// The image of the glyph.
    pub image: Image,

    /// The offset of the glyph.
    pub offset: Vector,

    /// The size of the glyph.
    pub size: Size,
}

/// A context for loading and rasterizing fonts.
///
/// This is a wrapper around the [`cosmic_text`] crate, and provides a simple interface for
/// loading and rasterizing fonts. Interacting with this directly is not necessary for most
/// applications, see [`Text`](crate::views::Text) and [`TextInput`](crate::views::TextInput).
#[derive(Debug)]
pub struct Fonts {
    /// The swash cache.
    pub swash_cache: SwashCache,
    /// The font system.
    pub font_system: FontSystem,
    /// The glyph cache.
    pub glyph_cache: HashMap<CacheKey, CachedGlyph>,
}

impl Default for Fonts {
    fn default() -> Self {
        Self::new()
    }
}

impl Fonts {
    /// Creates a new font context.
    pub fn new() -> Self {
        let swash_cache = SwashCache::new();

        let mut fonts = Vec::new();

        for font in EMBEDDED_FONTS {
            fonts.push(Source::Binary(Arc::new(font.to_vec())));
        }

        let mut font_system = FontSystem::new_with_fonts(fonts);
        let db = font_system.db_mut();

        db.set_serif_family("Roboto");
        db.set_sans_serif_family("Roboto");
        db.set_monospace_family("Roboto Mono");
        db.set_cursive_family("Roboto");
        db.set_fantasy_family("Roboto");

        Self {
            swash_cache,
            font_system,
            glyph_cache: HashMap::new(),
        }
    }

    /// Loads a font from a [`FontSource`].
    ///
    /// This will usually either be a path to a font file or the font data itself, but can also
    /// be a [`Vec<FontSource>`] to load multiple fonts at once.
    pub fn load_font(&mut self, source: impl Into<FontSource>) -> Result<(), io::Error> {
        match source.into() {
            FontSource::Data(data) => {
                self.font_system.db_mut().load_font_data(data);
            }
            FontSource::Path(path) => {
                self.font_system.db_mut().load_font_file(path)?;
            }
            FontSource::Set(sources) => {
                for source in sources {
                    self.load_font(source)?;
                }
            }
        }

        Ok(())
    }

    /// Loads the system fonts.
    ///
    /// This is a platform-specific operation, for more information see the
    /// documentation for [`fontdb::Database::load_system_fonts`](cosmic_text::fontdb::Database::load_system_fonts).
    pub fn load_system_fonts(&mut self) {
        self.font_system.db_mut().load_system_fonts();
    }

    /// Calculates the size of a text buffer.
    ///
    /// The resulting size is the smallest rectangle that can contain the text,
    /// and is roughly equal to the widest line and the line height multiplied
    /// the number of laid out lines.
    pub fn buffer_size(buffer: &Buffer) -> Size {
        let mut width = 0.0;
        let mut height = 0.0;

        for run in buffer.layout_runs() {
            width = f32::max(width, run.line_w);
            height += buffer.metrics().line_height;
        }

        Size::new(width, height).ceil()
    }

    /// Rasterize a glyph.
    pub fn rasterize_glyph(&mut self, glyph: &LayoutGlyph, scale: f32) -> &CachedGlyph {
        let physical = glyph.physical((0.0, 0.0), scale);

        if self.glyph_cache.contains_key(&physical.cache_key) {
            return self.glyph_cache.get(&physical.cache_key).unwrap();
        }

        let image = match self
            .swash_cache
            .get_image_uncached(&mut self.font_system, physical.cache_key)
        {
            Some(image) => image,
            None => panic!("failed to rasterize glyph"),
        };

        let data = match image.content {
            SwashContent::Mask => image.data.into_iter().flat_map(|a| [a, a, a, a]).collect(),
            SwashContent::SubpixelMask => todo!(),
            SwashContent::Color => image.data,
        };

        let size = Size::new(image.placement.width as f32, image.placement.height as f32) / scale;
        let offset = Vector::new(image.placement.left as f32, -image.placement.top as f32) / scale;

        let image = Image::new(data, image.placement.width, image.placement.height);

        let glyph = CachedGlyph {
            image,
            offset,
            size,
        };

        self.glyph_cache.insert(physical.cache_key, glyph);
        self.glyph_cache.get(&physical.cache_key).unwrap()
    }

    /// Rasterize a buffer.
    pub fn draw_buffer(
        &mut self,
        canvas: &mut Canvas,
        buffer: &Buffer,
        offset: Vector,
        scale: f32,
    ) {
        for run in buffer.layout_runs() {
            for glyph in run.glyphs {
                let cached = self.rasterize_glyph(glyph, scale);
                let physical = glyph.physical((0.0, 0.0), 1.0);

                let point = Point::new(physical.x as f32, run.line_y + physical.y as f32);

                let mut image = cached.image.clone();

                if let Some(color) = glyph.color_opt {
                    image.color(Color::rgba8(color.r(), color.g(), color.b(), color.a()));
                }

                canvas.image(point + cached.offset + offset, image);
            }
        }
    }
}
