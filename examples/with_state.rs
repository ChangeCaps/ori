use ori::prelude::*;

#[derive(Clone, Default)]
struct User {
    name: String,
    age: u8,
}

#[derive(Default)]
struct Data {
    users: Vec<User>,
}

fn form() -> impl View<Data> {
    // the `with_state` view is used to store state that is not part of the data
    with_state(User::default, |_data, user| {
        let name = text_input()
            .text(&user.name)
            .on_change(|_, (_, user): &mut (_, User), text| user.name = text);

        let age = hstack![
            text!("Age: {}", user.age),
            on_click(button(text("Add")), move |_, (_, user): &mut (_, User)| {
                user.age += 1
            })
        ];

        let submit = button(text("Submit")).color(style(Palette::ACCENT));

        let submit = on_click(submit, |_, (data, user): &mut (Data, User)| {
            data.users.push(user.clone());
            *user = User::default();
        });

        vstack![vstack![name, age], submit]
    })
}

fn app(data: &mut Data) -> impl View<Data> {
    let mut users = Vec::new();

    for user in data.users.iter_mut() {
        let fields = hstack![text!("Name: {},", user.name), text!("Age: {}", user.age)].gap(16.0);

        let user = container(pad(16.0, fields))
            .background(style(Palette::SECONDARY))
            .border_radius(8.0);

        users.push(center(user));
    }

    let users = pad(16.0, vscroll(vstack(users)));

    center(hstack![form(), users])
}

fn main() {
    let window = WindowDescriptor::new().title("With State (examples/with_state.rs)");
    Launcher::new(Data::default()).window(window, app).launch();
}
