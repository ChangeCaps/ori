use ori::prelude::*;

fn ui(_cx: Scope) -> impl View {
    network_image("https://i.imgur.com/7ZsEjTv.png")
}

fn main() {
    App::new(ui)
        .title("Network Image (examples/network_image.rs)")
        .run();
}
