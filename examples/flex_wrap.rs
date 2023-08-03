use ori::prelude::*;

fn content(cx: Scope) -> View {
    view! {
        <Div style:width=Px(100.0)
            style:height=Px(100.0)
            style:background=Color::RED />
    }
}

fn ui(cx: Scope) -> View {
    view! {
        <Div style:width=Vw(100.0)
            style:height=Vh(100.0)
            style:gap=Px(10.0)
            style:direction=Axis::Horizontal
            style:background=Color::BLACK
            style:flex-wrap=FlexWrap::WrapReverse>
            { (0..15).map(move |_| content(cx)).collect::<Vec<_>>() }
        </Div>
    }
}

fn main() {
    App::new(ui).run();
}
