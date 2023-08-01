//! This is the example from the readme.
use ori::prelude::*;

// define the ui
fn ui(cx: Scope) -> View {
    // create a signal that will hold the state of the counter
    let counter = cx.signal(0);

    // render the ui using the view! macro
    view! {
        <Button on:click=move |_| *counter.modify() += 1>
            "Click me!"
        </Button>
        { format!("Clicked {} times", counter.get()) }
    }
}

fn main() {
    // configure and start the application
    App::new(ui).run();
}
