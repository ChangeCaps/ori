use ori::prelude::*;

fn ui(_cx: Scope) -> impl View {
    let mut row = Stack::vstack();

    for _ in 0..40 {
        let mut column = Stack::hstack();

        for _ in 0..40 {
            column.push(Button::new(Text::new("A")));
        }

        row.push(column);
    }

    row
}

fn main() {
    App::new(ui).run();
}
