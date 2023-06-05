use std::f32::consts::PI;

use ori::prelude::*;

#[derive(Default, Build)]
struct CustomElement {}

impl Element for CustomElement {
    type State = ();

    fn build(&self) -> Self::State {}

    // set the style of the view, with a custom element name "custom-element"
    fn style(&self) -> Style {
        Style::new("custom-element")
    }

    // layout the element
    fn layout(
        &self,
        _state: &mut Self::State,
        _cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        // this view will take all the available space
        space.max
    }

    fn draw(&self, _state: &mut Self::State, cx: &mut DrawContext) {
        // use the "custom-radius" style property
        // here we use a style_range, because the attribute is a Length,
        // this means we can have percentage values and other units
        let radius = cx.style_range("custom-radius", 5.0..100.0);

        // create a parametric curve
        let curve = Curve::parametric(
            |t| {
                let t = t * 2.0;
                let x = t.sin();
                let y = t.cos();
                cx.rect().center() + Vec2::new(x, y) * radius
            },
            0.0,
            PI,
        );

        // fill the curve with a blue color
        cx.draw(curve.fill(Color::BLUE));
    }
}

fn ui(_cx: Scope) -> View {
    view! {
        <CustomElement />
    }
}

fn main() {
    App::new(ui)
        .style(stylesheet!("examples/style/custom-element.css"))
        .run();
}
