use ori::prelude::*;

fn ui(cx: Scope) -> impl View {
    let text = signal(cx, String::new());

    let input = TextInput::new(text)
        .width(Em(10.0))
        .multiline(true)
        .on_submit(|text| info!("Submit: `{}`", text));

    let clear = Button::floating("Clear").on_press(move |_| text.set(String::new()));

    Align::center(hstack![input, clear].align_items(AlignItems::Center))
}

fn main() {
    App::new(ui)
        .title("Text Input (examples/text_input.rs)")
        .run();
}
