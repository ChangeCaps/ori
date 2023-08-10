use std::any::{Any, TypeId};

use deref_derive::{Deref, DerefMut};
use glam::Vec2;
use ori_graphics::{Affine, Frame, Glyphs, Primitive, PrimitiveKind, Rect};
use ori_reactive::Event;

use crate::{AvailableSpace, Context, Padding};

#[derive(Deref, DerefMut)]
pub struct EventContext<'a> {
    #[deref]
    pub(crate) context: Context<'a>,
    pub(crate) transform: Affine,
}

impl<'a> EventContext<'a> {
    pub(crate) fn new(context: Context<'a>, transform: Affine) -> Self {
        Self { context, transform }
    }

    pub fn size(&self) -> Vec2 {
        self.context.size()
    }

    pub fn rect(&self) -> Rect {
        Rect::new(Vec2::ZERO, self.size())
    }

    pub fn transform(&self) -> Affine {
        self.transform
    }

    /// Transforms a point from global coordinates to local coordinates.
    ///
    /// This is useful when dealing with mouse events.
    pub fn local(&self, point: Vec2) -> Vec2 {
        self.transform.inverse() * point
    }

    /// Transforms the context for the duration of the closure.
    pub fn with_transform<T>(
        &mut self,
        transform: Affine,
        f: impl FnOnce(&mut EventContext<'_>) -> T,
    ) -> T {
        let old_transform = self.transform;
        self.transform *= transform;
        let result = f(self);
        self.transform = old_transform;
        result
    }

    /// Translates the context for the duration of the closure.
    pub fn with_translation<T>(
        &mut self,
        translation: Vec2,
        f: impl FnOnce(&mut EventContext<'_>) -> T,
    ) -> T {
        self.with_transform(Affine::translation(translation), f)
    }

    /// Pads the context for the duration of the closure.
    pub fn with_padding<T>(
        &mut self,
        padding: Padding,
        f: impl FnOnce(&mut EventContext<'_>) -> T,
    ) -> T {
        self.with_translation(padding.translation(self), f)
    }
}

#[derive(Deref, DerefMut)]
pub struct LayoutContext<'a> {
    #[deref]
    pub(crate) context: Context<'a>,
}

impl<'a> LayoutContext<'a> {
    pub(crate) fn new(context: Context<'a>) -> Self {
        Self { context }
    }
}

#[derive(Deref, DerefMut)]
pub struct DrawContext<'a> {
    #[deref]
    pub(crate) context: Context<'a>,
    pub(crate) frame: &'a mut Frame,
}

impl<'a> DrawContext<'a> {
    pub(crate) fn new(context: Context<'a>, frame: &'a mut Frame) -> Self {
        Self { context, frame }
    }

    pub fn size(&self) -> Vec2 {
        self.context.size()
    }

    pub fn rect(&self) -> Rect {
        Rect::new(Vec2::ZERO, self.size())
    }

    pub fn transform(&self) -> Affine {
        self.frame.transform
    }

    pub fn z_index(&self) -> f32 {
        self.frame.z_index
    }

    pub fn clip(&self) -> Option<Rect> {
        self.frame.clip
    }

    pub fn with_transform<T>(
        &mut self,
        transform: Affine,
        f: impl FnOnce(&mut DrawContext<'_>) -> T,
    ) -> T {
        let old_transform = self.frame.transform;
        self.frame.transform *= transform;
        let result = f(self);
        self.frame.transform = old_transform;
        result
    }

    pub fn with_translation<T>(
        &mut self,
        translation: Vec2,
        f: impl FnOnce(&mut DrawContext<'_>) -> T,
    ) -> T {
        self.with_transform(Affine::translation(translation), f)
    }

    pub fn with_padding<T>(
        &mut self,
        padding: Padding,
        f: impl FnOnce(&mut DrawContext<'_>) -> T,
    ) -> T {
        self.with_translation(padding.translation(self), f)
    }

    pub fn with_layer<T>(&mut self, z_index: f32, f: impl FnOnce(&mut DrawContext<'_>) -> T) -> T {
        let old_z_index = self.frame.z_index;
        self.frame.z_index += z_index;
        let result = f(self);
        self.frame.z_index = old_z_index;
        result
    }

    pub fn with_clip<T>(&mut self, rect: Rect, f: impl FnOnce(&mut DrawContext<'_>) -> T) -> T {
        let old_clip = self.frame.clip;
        self.frame.clip = Some(rect);
        let result = f(self);
        self.frame.clip = old_clip;
        result
    }

    pub fn draw(&mut self, primitive: impl Into<PrimitiveKind>) {
        self.frame.draw(primitive);
    }

    pub fn draw_text(&mut self, glyphs: &Glyphs, rect: Rect) {
        let mesh = self.context.fonts.text_mesh(self.renderer, glyphs, rect);

        if let Some(mesh) = mesh {
            self.frame.draw_primitive(Primitive {
                kind: PrimitiveKind::Mesh(mesh),
                z_index: self.frame.z_index,
                transform: self.frame.transform.round(),
                clip: self.frame.clip,
            });
        }
    }
}

pub trait IntoView: Sized {
    type View: View;

    fn into_view(self) -> Self::View;
}

impl<T: View> IntoView for T {
    type View = Self;

    fn into_view(self) -> Self::View {
        self
    }
}

#[allow(unused_variables)]
pub trait View: Any + Send + Sync {
    fn event(&self, cx: &mut EventContext<'_>, event: &Event) {}

    fn layout(&self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        space.min
    }

    fn draw(&self, cx: &mut DrawContext<'_>) {}
}

impl dyn View {
    pub fn downcast_ref<T: View>(&self) -> Option<&T> {
        if <dyn View>::type_id(self) == TypeId::of::<T>() {
            // SAFETY: `T` is the same type as `self`.
            unsafe { Some(&*(self as *const dyn View as *const T)) }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: View>(&mut self) -> Option<&mut T> {
        if <dyn View>::type_id(self) == TypeId::of::<T>() {
            // SAFETY: `T` is the same type as `self`.
            unsafe { Some(&mut *(self as *mut dyn View as *mut T)) }
        } else {
            None
        }
    }
}

impl View for () {}
