//! This is the example from the readme.
use ori::prelude::*;

// define the ui
fn ui(cx: Scope) -> impl View {
    // create a signal that will hold the state of the counter
    let counter = signal(cx, 0);

    let text = dynamic(cx, move |_| {
        Text::new(format!("Clicked {} times", counter.get()))
    });

    let button = Button::new(text).on_press(move |_| *counter.modify() += 1);

    row![FlexChild::new(button).flex(1.0)]
}

fn main() {
    // configure and start the application
    App::new(ui).run();
}
