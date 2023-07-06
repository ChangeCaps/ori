use glam::Vec2;
use ori_macro::Build;
use ori_style::Style;

use crate::{AvailableSpace, DrawContext, Element, LayoutContext};

/// A horizontal spacer.
#[derive(Clone, Debug, Default, Build)]
pub struct HSpacer;

impl Element for HSpacer {
    type State = ();

    fn build(&self) -> Self::State {}

    fn style(&self) -> Style {
        Style::new("hspacer")
    }

    fn layout(
        &self,
        _state: &mut Self::State,
        _cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        space.max
    }

    fn draw(&self, _state: &mut Self::State, cx: &mut DrawContext) {
        cx.draw_quad();
    }
}

/// A vertical spacer.
#[derive(Clone, Debug, Default, Build)]
pub struct VSpacer;

impl Element for VSpacer {
    type State = ();

    fn build(&self) -> Self::State {}

    fn style(&self) -> Style {
        Style::new("vspacer")
    }

    fn layout(
        &self,
        _state: &mut Self::State,
        _cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        space.max
    }

    fn draw(&self, _state: &mut Self::State, cx: &mut DrawContext) {
        cx.draw_quad();
    }
}
