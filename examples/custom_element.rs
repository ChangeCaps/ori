use std::f32::consts::PI;

use ori::prelude::*;

#[derive(Default, Build)]
struct CustomElement {}

impl Element for CustomElement {
    type State = f32;

    fn build(&self) -> Self::State {
        0.0
    }

    // set the style of the view, with a custom element name "custom-element"
    fn style(&self) -> Style {
        Style::new("custom-element")
    }

    // layout the element
    fn layout(
        &self,
        radius: &mut Self::State,
        cx: &mut LayoutContext,
        _space: AvailableSpace,
    ) -> Vec2 {
        // use the "custom-radius" style property
        // here we use a style_range, because the attribute is a Length,
        // this means we can have percentage values and other units
        *radius = cx.style_length("custom-radius", 5.0..100.0);

        // this view will take up a rectangular space with the size of the radius
        Vec2::splat(*radius * 2.0)
    }

    fn draw(&self, radius: &mut Self::State, cx: &mut DrawContext) {
        // create a parametric curve
        let curve = Curve::parametric(
            |t| {
                let t = t * 2.0;
                let x = t.sin();
                let y = t.cos();
                cx.global_rect().center() + Vec2::new(x, y) * *radius
            },
            0.0,
            PI,
        );

        // fill the curve with a color
        let color = cx.style("custom-color");
        cx.draw(curve.fill(color))
    }
}

fn ui(_cx: Scope) -> View {
    view! {
        <CustomElement />
    }
}

fn main() {
    App::new(ui)
        .style(style!("examples/style/custom-element.css"))
        .run();
}
