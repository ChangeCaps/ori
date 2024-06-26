use ori::prelude::*;

const ATTRIBUTION: &str = "\"Peony flowers (1843)\" by Swallowtail Garden Seeds is marked with Public Domain Mark 1.0. To view the terms, visit https://creativecommons.org/publicdomain/mark/1.0/?ref=openverse.";

struct Data;

fn app(_data: &mut Data) -> impl View<Data> {
    center(vstack![
        flex(include_image!("examples/flowers.jpg")),
        max_width(300.0, text(ATTRIBUTION)),
    ])
}

fn main() {
    let window = Window::new().title("Image (examples/image.rs)");

    let app = App::build().window(window, app);

    ori::launch(app, Data).unwrap();
}
