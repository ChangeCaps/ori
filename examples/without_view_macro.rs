use ori::prelude::*;
use ori_core::BindCallback;

fn ui(cx: Scope) -> impl IntoView {
    let counter = signal(cx, 0);

    let mut button = Button::new("Click me!");
    button.on_click.bind(cx, move |_| *counter.modify() += 1);

    let counter_text = dynamic(cx, move |_| format!("Clicked {} times", counter.get()));

    (button, counter_text)
}

fn main() {
    App::new(ui).run();
}
