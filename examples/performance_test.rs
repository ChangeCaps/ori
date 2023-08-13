use ori::prelude::*;

fn ui(_cx: Scope) -> impl View {
    let mut row = Stack::hstack();

    for _ in 0..2000 {
        row.push(Button::new(Icon::new("steam")));
    }

    Align::center(row)
}

fn main() {
    App::new(ui).run();
}
