use ori::prelude::*;

pub const BORDER_TOP: Key<BorderWidth> = Key::new("todos.border-top");

static TODOS: AtomRef<Vec<Todo>> = atom!(ref Vec::new());
static SELECTION: Atom<Selection> = atom!(Selection::All);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Selection {
    All,
    Active,
    Completed,
}

#[derive(Clone, Debug, Default)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}

fn title(_cx: Scope) -> Text {
    Text::new("todos").font_size(Px(50.0))
}

fn input(cx: Scope) -> impl View {
    let text = signal(cx, String::new());

    let input = TextInput::new(text)
        .placeholder("What needs to be done?")
        .font_size(Px(20.0))
        .on_submit(move |title| {
            if !title.is_empty() {
                TODOS.write().push(Todo {
                    title: String::from(title),
                    completed: false,
                });

                text.set(String::new());
            }
        });

    Container::new(input)
        .padding((Em(4.0), Em(1.0)))
        .width(Em(28.0))
}

fn todo(_cx: Scope, index: usize) -> impl View {
    let todo = TODOS.read()[index].clone();

    let completed = CheckBox::new(todo.completed)
        .on_press(move |_| {
            let todo = &mut TODOS.write()[index];
            todo.completed = !todo.completed;
        })
        .color(Palette::ACCENT);

    let title = Text::new(&todo.title)
        .font_size(Px(20.0))
        .color(if todo.completed {
            Palette::TEXT_BRIGHTER
        } else {
            Palette::TEXT
        });

    let close = Button::fancy(Icon::new("xmark"))
        .on_press(move |_| {
            TODOS.write().remove(index);
        })
        .color(Color::hsl(0.0, 0.7, 0.7));

    let left = hstack![completed, title]
        .align_items(AlignItems::Center)
        .gap(Em(1.5));

    let items = hstack![left, close]
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::SpaceBetween);

    Container::new(items)
        .padding(Em(1.0))
        .border_width(BORDER_TOP)
        .width(Em(28.0))
}

fn todos(cx: Scope) -> impl View {
    let mut todos = Stack::column();

    for (i, item) in TODOS.read().iter().enumerate().rev() {
        match SELECTION.get() {
            Selection::Active if item.completed => continue,
            Selection::Completed if !item.completed => continue,
            _ => {}
        }

        todos.push(todo(cx, i));
    }

    todos.gap(0.0)
}

fn active_count(_cx: Scope) -> impl View {
    let active = TODOS.read().iter().filter(|todo| !todo.completed).count();

    let active_text = if active == 1 {
        String::from("1 item left")
    } else {
        format!("{} items left", active)
    };

    Text::new(active_text).font_size(Em(0.8))
}

fn selection(cx: Scope) -> impl View {
    if TODOS.read().is_empty() {
        return None;
    }

    let all = Button::fancy("All")
        .on_press(move |_| {
            SELECTION.set(Selection::All);
        })
        .color(if SELECTION.get() == Selection::All {
            Palette::ACCENT
        } else {
            Palette::PRIMARY
        })
        .padding((Em(0.3), Em(0.2)));

    let active = Button::fancy("Active")
        .on_press(move |_| {
            SELECTION.set(Selection::Active);
        })
        .color(if SELECTION.get() == Selection::Active {
            Palette::ACCENT
        } else {
            Palette::PRIMARY
        })
        .padding((Em(0.3), Em(0.2)));

    let completed = Button::fancy("Completed")
        .on_press(move |_| {
            SELECTION.set(Selection::Completed);
        })
        .color(if SELECTION.get() == Selection::Completed {
            Palette::ACCENT
        } else {
            Palette::PRIMARY
        })
        .padding((Em(0.3), Em(0.2)));

    let stack = hstack![active_count(cx), all, active, completed]
        .justify_content(JustifyContent::SpaceAround)
        .align_items(AlignItems::Center)
        .gap(Em(1.0));

    Some(
        Container::new(stack)
            .width(Em(26.0))
            .padding(Em(0.5))
            .border_width(BORDER_TOP),
    )
}

fn ui(cx: Scope) -> impl View {
    Align::new(
        (0.5, 0.2),
        vstack![
            title(cx),
            vstack![input(cx), react!(todos(cx)), react!(selection(cx))]
                .align_items(AlignItems::Center)
                .gap(0.0),
        ]
        .align_items(AlignItems::Center)
        .gap(Em(1.0)),
    )
}

fn theme() -> Theme {
    let mut theme = Theme::new();

    theme.set(Container::BACKGROUND_COLOR, Palette::BACKGROUND_DARK);
    theme.set(Container::BORDER_WIDTH, BorderWidth::all(0.0));
    theme.set(Container::BORDER_RADIUS, BorderRadius::all(0.0));
    theme.set(Container::BORDER_COLOR, Palette::SECONDARY_DARK);

    theme.set(BORDER_TOP, BorderWidth::new(1.0, 0.0, 0.0, 0.0));

    theme.set(CheckBox::BORDER_WIDTH, BorderWidth::all(1.0));
    theme.set(CheckBox::BORDER_RADIUS, BorderRadius::all(Em(0.75)));

    theme
}

fn main() {
    App::new(ui)
        .title("Todos (examples/todos.rs)")
        .theme(theme())
        .run();
}
