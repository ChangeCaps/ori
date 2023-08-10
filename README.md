# Ori
[![Crates.io](https://img.shields.io/crates/v/ori)](https://crates.io/crates/ori)
[![Documentation](https://img.shields.io/docsrs/ori)](https://docs.rs/ori/latest)

Ori is a reactive ui framework for creating native applications for rust.

```rust
use ori::prelude::*;

// define the ui
fn ui(cx: Scope) -> impl View {
    // create a signal that will hold the state of the counter
    let counter = signal(cx, 0);

    // we use the reactive! macro to create a reactive ui component
    let text = reactive!(format!("Clicked {} times", counter.get()));

    // we create a button that increments the counter when pressed
    let button = Button::new(text).on_press(move |_| *counter.modify() += 1);

    // we center the button in the window
    Align::center(button)
}

fn main() {
    // configure and run the application
    App::new(ui).title("Readme (examples/readme.rs)").run();
}
```

## Examples
A [`calculator`](examples/calculator.rs) made with ori.

![Calculator image](assets/calculator.png)

## License
Ori is dual-licensed under either:
 - MIT
 - Apache License, Version 2.0
