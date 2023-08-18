use glam::Vec2;

use crate::{
    Affine, BorderRadius, BorderWidth, Color, Fragment, Primitive, Quad, Rect, Scene, Size,
};

/// A canvas used for drawing a [`Scene`].
pub struct Canvas<'a> {
    scene: &'a mut Scene,
    /// The transform to apply to the canvas.
    pub transform: Affine,
    /// The depth of the canvas.
    pub depth: f32,
    /// The clip rectangle of the canvas.
    pub clip: Rect,
}

impl<'a> Canvas<'a> {
    /// Create a new [`Canvas`].
    pub fn new(scene: &'a mut Scene, window_size: Size) -> Self {
        Self {
            scene,
            transform: Affine::IDENTITY,
            depth: 0.0,
            clip: Rect::min_size(Vec2::ZERO, window_size),
        }
    }

    /// Create a new layer.
    pub fn layer(&mut self) -> Canvas<'_> {
        Canvas {
            scene: self.scene,
            transform: self.transform,
            depth: self.depth + 1.0,
            clip: self.clip,
        }
    }

    /// Translate the canvas.
    pub fn transform(&mut self, transform: Affine) {
        self.transform *= transform;
    }

    /// Translate the canvas.
    pub fn translate(&mut self, translation: Vec2) {
        self.transform *= Affine::translate(translation);
    }

    /// Rotate the canvas.
    pub fn rotate(&mut self, angle: f32) {
        self.transform *= Affine::rotate(angle);
    }

    /// Scale the canvas.
    pub fn scale(&mut self, scale: Vec2) {
        self.transform *= Affine::scale(scale);
    }

    /// Draw a fragment to the canvas.
    ///
    /// This is the lowest-level drawing method, and will not apply any
    /// of `transform`, `depth`, or `clip`. These must be set manually.
    pub fn draw_fragment(&mut self, fragment: Fragment) {
        self.scene.push(fragment);
    }

    /// Draw a primitive to the canvas.
    pub fn draw(&mut self, primitive: impl Into<Primitive>) {
        self.draw_fragment(Fragment {
            primitive: primitive.into(),
            transform: self.transform,
            depth: self.depth,
            clip: self.clip,
        });
    }

    /// Draw a [`Primitive`] with pixel-perfect coordinates.
    pub fn draw_pixel_perfect(&mut self, primitive: impl Into<Primitive>) {
        self.draw_fragment(Fragment {
            primitive: primitive.into(),
            transform: self.transform.round(),
            depth: self.depth,
            clip: self.clip,
        });
    }

    /// Draw a quad to the canvas.
    pub fn draw_quad(
        &mut self,
        rect: Rect,
        color: impl Into<Color>,
        border_radius: impl Into<BorderRadius>,
        border_width: impl Into<BorderWidth>,
        border_color: impl Into<Color>,
    ) {
        self.draw(Quad {
            rect,
            color: color.into(),
            border_radius: border_radius.into(),
            border_width: border_width.into(),
            border_color: border_color.into(),
        });
    }
}