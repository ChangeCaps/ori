use std::any::{Any, TypeId};

use deref_derive::{Deref, DerefMut};
use ori_graphics::{math::Vec2, Affine, Frame, Glyphs, Primitive, PrimitiveKind, Rect};
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

    pub fn borrow(&mut self) -> EventContext<'_> {
        EventContext {
            context: self.context.borrow(),
            transform: self.transform,
        }
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
        self.with_transform(Affine::translate(translation), f)
    }

    /// Pads the context for the duration of the closure.
    pub fn with_padding<T>(
        &mut self,
        padding: Padding,
        f: impl FnOnce(&mut EventContext<'_>) -> T,
    ) -> T {
        self.with_translation(padding.offset(self), f)
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

    pub fn borrow(&mut self) -> LayoutContext<'_> {
        LayoutContext {
            context: self.context.borrow(),
        }
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

    pub fn borrow(&mut self) -> DrawContext<'_> {
        DrawContext {
            context: self.context.borrow(),
            frame: self.frame,
        }
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
        self.with_transform(Affine::translate(translation), f)
    }

    pub fn with_padding<T>(
        &mut self,
        padding: Padding,
        f: impl FnOnce(&mut DrawContext<'_>) -> T,
    ) -> T {
        self.with_translation(padding.offset(self), f)
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

    pub fn draw_rounded(&mut self, primitive: impl Into<PrimitiveKind>) {
        self.frame.draw_rounded(primitive);
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

/// A view is a widget that can be drawn on the screen.
pub trait View: Any + Send {
    /// Handle an event.
    fn event(&mut self, cx: &mut EventContext<'_>, event: &Event);

    /// Compute the layout of the view.
    fn layout(&mut self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2;

    /// Draw the view.
    fn draw(&mut self, cx: &mut DrawContext<'_>);
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

impl View for () {
    fn event(&mut self, _cx: &mut EventContext<'_>, _event: &Event) {}

    fn layout(&mut self, _cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        space.min
    }

    fn draw(&mut self, _cx: &mut DrawContext<'_>) {}
}

impl View for Box<dyn View> {
    fn event(&mut self, cx: &mut EventContext<'_>, event: &Event) {
        self.as_mut().event(cx, event);
    }

    fn layout(&mut self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        self.as_mut().layout(cx, space)
    }

    fn draw(&mut self, cx: &mut DrawContext<'_>) {
        self.as_mut().draw(cx);
    }
}

impl<T: View> View for Option<T> {
    fn event(&mut self, cx: &mut EventContext<'_>, event: &Event) {
        if let Some(this) = self {
            this.event(cx, event);
        }
    }

    fn layout(&mut self, cx: &mut LayoutContext<'_>, space: AvailableSpace) -> Vec2 {
        if let Some(this) = self {
            this.layout(cx, space)
        } else {
            space.min
        }
    }

    fn draw(&mut self, cx: &mut DrawContext<'_>) {
        if let Some(this) = self {
            this.draw(cx);
        }
    }
}
