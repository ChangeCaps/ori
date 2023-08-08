use ori::prelude::*;

fn content(cx: Scope) -> IntoView {
    view! {
        <Div style:width=Px(100.0)
            style:height=Px(100.0)
            style:background=Color::RED />
    }
}

fn ui(cx: Scope) -> IntoView {
    view! {
        <Scroll style:width=Pc(100.0)
            style:align-items=AlignItem::Center
            style:background=Color::CYAN>
            <Div style:gap=Px(10.0)
                style:direction=Axis::Horizontal
                style:background=Color::BLACK
                style:flex-wrap=FlexWrap::WrapReverse>
                { (0..2).map(move |_| content(cx)).collect::<Vec<_>>() }
            </Div>
        </Scroll>
    }
}

fn main() {
    App::new(ui).run();
}
