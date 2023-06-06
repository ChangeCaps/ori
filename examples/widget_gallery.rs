use ori::prelude::*;

const LONG_TEXT: &str = include_str!("long_text.txt");

fn popup_ui(cx: Scope) -> View {
    // use an Atom to store the counter value, so that it persists when popup closes
    static COUNTER: Atom<i32> = atom!(0);

    view! {
        <Div class="widget-gallery">
            <Div class="popup">
                <Button on:click=move |_| *COUNTER.modify() += 1>
                    "Click me!"
                </Button>
                { format!("Clicked {} times", COUNTER.get()) }
            </Div>
        </Div>
    }
}

fn ui(cx: Scope) -> View {
    let counter = cx.signal(1);
    let checked = cx.signal(false);
    let knob_value = cx.signal(0.0);
    let long_text = cx.signal(String::from(LONG_TEXT));
    let text = cx.signal(String::new());
    let radio_button = cx.signal(0);

    // popup state
    let popup_window = cx.signal(None);

    // when popup_open changes, open or close the popup window
    let toggle_popup = move |_: &PointerEvent| {
        info!("toggle popup");

        if popup_window.get().is_none() {
            let window = Window::new().title("Widget Gallery Popup").size(300, 300);
            let id = cx.open_window(window, popup_ui);

            popup_window.set(Some(id));
        } else if let Some(id) = popup_window.get() {
            cx.emit(CloseWindow::window(id));
            popup_window.set(None);
        }
    };

    cx.on(move |_: &WindowClosedEvent| {
        popup_window.set(None);
    });

    let text_size = cx.memo(move || if checked.get() { Em(2.0) } else { Em(1.5) });
    let on_click = move |_: &PointerEvent| *counter.modify() += 1;

    view! {
        <Div class="widget-gallery">
            <Div class="column">
                <Div class="row">
                    <Text text="Toggle me" style:font-size=trans(text_size.get(), 0.25) />
                    <Checkbox bind:checked=checked />
                </Div>

                <Div class="row" style:justify-content=JustifyContent::End>
                    "Popup"
                    <Checkbox checked=popup_window.get().is_some() on:click=toggle_popup />
                </Div>

                <Button on:click=on_click>
                    { format!("Counter: {}", counter.get()) }
                </Button>

                <Image src="examples/images/image.jpg" />

                <TextInput bind:text=text on:input=|text| info!("Input '{}'", text) />

                { format!("Input: {}", text.get()) }
            </Div>
            <Scroll style:max-height=Em(18.0)>
                <Div style:max-width=Em(15.0)>
                    <TextInput class="long-text" bind:text=long_text multiline=true />
                </Div>
            </Scroll>
            <Div class="column">
                <Knob bind:value=knob_value max=2.0 />
                <Text style:text-align=TextAlign::Center text=format!("{:.2}", knob_value.get()) />
            </Div>
            <Slider style:direction=Axis::Vertical style:height=Em(10.0) min=-1.0 bind:value=knob_value />
            <Div class="column">
                <Div class="row" style:justify-content=JustifyContent::Start>
                    <Radio selected=radio_button.get()==0 on:click=move |_| radio_button.set(0) />
                    "Hello"
                </Div>
                <Div class="row" style:justify-content=JustifyContent::Start>
                    <Radio selected=radio_button.get()==1 on:click=move |_| radio_button.set(1) />
                    "Here"
                </Div>
                <Div class="row" style:justify-content=JustifyContent::Start>
                    <Radio selected=radio_button.get()==2 on:click=move |_| radio_button.set(2) />
                    "Are"
                </Div>
                <Div class="row" style:justify-content=JustifyContent::Start>
                    <Radio selected=radio_button.get()==3 on:click=move |_| radio_button.set(3) />
                    "Some"
                </Div>
                <Div class="row" style:justify-content=JustifyContent::Start>
                    <Radio selected=radio_button.get()==4 on:click=move |_| radio_button.set(4) />
                    "Radio"
                </Div>
                <Div class="row" style:justify-content=JustifyContent::Start>
                    <Radio selected=radio_button.get()==5 on:click=move |_| radio_button.set(5) />
                    "Buttons"
                </Div>
                <Div class="row" style:justify-content=JustifyContent::Start>
                    <Radio selected=radio_button.get()==6 on:click=move |_| radio_button.set(6) />
                    { format!("{}", radio_button.get()) }
                </Div>
            </Div>
        </Div>
    }
}

fn main() {
    App::new(ui) // create a new app with the ui function
        .title("Widget Gallery (examples/widget_gallery.rs)") // set the window title
        .night_theme()
        .style(style!("examples/style/widget-gallery.css")) // load a custom stylesheet
        .run(); // run the app
}
