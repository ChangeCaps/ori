//! This is the example from README.md.

use ori::prelude::*;

// define the ui
fn ui(cx: Scope) -> impl View {
    // create a signal that will hold the state of the counter
    let counter = signal(cx, 0);
    let checked = signal(cx, true);

    // we use the reactive! macro to create a reactive ui component
    let text = reactive!(format!("Clicked {} times", counter.get()));

    // we create a button that increments the counter when pressed
    let button = Button::new(text).on_press(move |_| *counter.modify() += 1);

    // we center the button in the window
    Align::center(hstack![button, CheckBox::new(checked)])
}

fn main() {
    // configure and run the application
    App::new(ui).title("Readme (examples/readme.rs)").run();
}
