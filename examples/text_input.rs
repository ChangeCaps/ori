use ori::prelude::*;

fn toggle_multiline(_cx: Scope, multiline: Signal<bool>) -> impl View {
    hstack![Text::new("Multiline"), CheckBox::new(multiline)]
}

fn input(cx: Scope, text: Signal<String>, multiline: Signal<bool>) -> impl View {
    react! {
        let input = TextInput::new(text)
            .min_width(Em(10.0))
            .multiline(multiline.get())
            .on_submit(|text| info!("Submit: `{}`", text));
        Container::new(input).padding(Em(0.5))
    }
}

fn ui(cx: Scope) -> impl View {
    let text = signal(cx, String::new());
    let multiline = signal(cx, false);

    let input = input(cx, text, multiline);
    let clear = Button::fancy("Clear").on_press(move |_| text.set(String::new()));

    Align::center(vstack![
        toggle_multiline(cx, multiline),
        hstack![input, clear],
    ])
}

fn main() {
    App::new(ui)
        .title("Text Input (examples/text_input.rs)")
        .run();
}
