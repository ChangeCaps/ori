use crate::{Affine, Mesh, Quad, Rect};

/// A primitive that can be drawn to the screen, see [`Primitive`] for more information.
pub enum PrimitiveKind {
    Quad(Quad),
    Mesh(Mesh),
}

impl From<Quad> for PrimitiveKind {
    fn from(quad: Quad) -> Self {
        Self::Quad(quad)
    }
}

impl From<Mesh> for PrimitiveKind {
    fn from(mesh: Mesh) -> Self {
        Self::Mesh(mesh)
    }
}

/// A primitive that can be drawn to the screen, see [`Frame`] for more information.
///
/// Primitives are drawn in order of their z-index, with primitives with a higher z-index being
/// drawn on top of primitives with a lower z-index. Primitives with the same z-index are drawn in
/// the order they are added to the frame.
///
/// Primitives can be clipped to a rectangle, see [`Frame::clip`] for more information.
pub struct Primitive {
    pub kind: PrimitiveKind,
    pub z_index: f32,
    pub transform: Affine,
    pub clip: Option<Rect>,
}

/// A collection of primitives that can be drawn to the screen.
#[derive(Default)]
pub struct Frame {
    primitives: Vec<Primitive>,
    pub z_index: f32,
    pub transform: Affine,
    pub clip: Option<Rect>,
}

impl Frame {
    /// Create a new frame.
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
            z_index: 0.0,
            transform: Affine::IDENTITY,
            clip: None,
        }
    }

    /// Clear the frame.
    pub fn clear(&mut self) {
        self.primitives.clear();
        self.z_index = 0.0;
        self.transform = Affine::IDENTITY;
        self.clip = None;
    }

    /// Get the z-index of the frame.
    pub fn z_index(&self) -> f32 {
        self.z_index
    }

    /// Get the clipping rectangle of the frame.
    pub fn clip(&self) -> Option<Rect> {
        self.clip
    }

    /// Draw a [`PrimitiveKind`] to the frame.
    pub fn draw(&mut self, primitive: impl Into<PrimitiveKind>) {
        self.draw_primitive(Primitive {
            kind: primitive.into(),
            z_index: self.z_index,
            transform: self.transform,
            clip: self.clip,
        });
    }

    /// Draw a [`PrimitiveKind`] to the frame, with a rounded transform.
    pub fn draw_rounded(&mut self, primitive: impl Into<PrimitiveKind>) {
        self.draw_primitive(Primitive {
            kind: primitive.into(),
            z_index: self.z_index,
            transform: self.transform.round(),
            clip: self.clip,
        });
    }

    /// Draw a [`Primitive`] to the frame.
    pub fn draw_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }

    /// Draws a [`Layer`] to the frame.
    pub fn layer(&mut self) -> Layer<'_> {
        Layer {
            frame: self,
            z_index: 1.0,
            transform: Affine::IDENTITY,
            clip: None,
        }
    }

    /// Draws a [`Layer`] to the frame.
    pub fn draw_layer(&mut self, f: impl FnOnce(&mut Self)) {
        self.layer().draw(f);
    }

    /// Get the primitives in the frame.
    pub fn primitives(&self) -> &[Primitive] {
        &self.primitives
    }
}

/// A layer is a frame with a z-index and a clipping rectangle, usually the z-index is one greater
/// than the z-index of the frame it is drawn to.
pub struct Layer<'a> {
    frame: &'a mut Frame,
    pub z_index: f32,
    pub transform: Affine,
    pub clip: Option<Rect>,
}

impl<'a> Layer<'a> {
    /// Set the z-index of the layer.
    pub fn z_index(mut self, z_index: f32) -> Self {
        self.z_index = z_index;
        self
    }

    /// Transform the layer;
    pub fn transform(mut self, transform: Affine) -> Self {
        self.transform *= transform;
        self
    }

    /// Set the clipping rectangle of the layer.
    pub fn clip(mut self, clip: impl Into<Option<Rect>>) -> Self {
        self.clip = clip.into().map(Rect::round);
        self
    }

    /// Draw to the layer, with `f` being called with a [`Frame`].
    pub fn draw(self, f: impl FnOnce(&mut Frame)) {
        self.frame.z_index += self.z_index;
        let tmp_transform = self.frame.transform;

        self.frame.transform *= self.transform;
        if let Some(clip) = self.clip {
            // save the old clip
            let tmp_clip = self.frame.clip;

            // set the new clip intersected with the old clip
            self.frame.clip = match tmp_clip {
                Some(old_clip) => Some(old_clip.intersect(clip)),
                None => Some(clip),
            };

            // draw to the frame
            f(self.frame);

            // restore the old clip
            self.frame.clip = tmp_clip;
        } else {
            f(self.frame);
        }

        self.frame.z_index -= self.z_index;
        self.frame.transform = tmp_transform;
    }
}
