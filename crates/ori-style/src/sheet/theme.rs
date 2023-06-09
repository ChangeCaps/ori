macro_rules! theme {
    ($name:ident, $folder:literal => $($style:literal),* $(,)?) => {
        #[allow(missing_docs)]
        pub const $name: &str = concat!(
            $(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/style/", $folder, "/", $style))),*
        );
    };
}

theme!(
    DAY_THEME,
    "day" =>
    "default.css",
    "body.css",
    "button.css",
    "checkbox.css",
    "combobox.css",
    "div.css",
    "knob.css",
    "scroll.css",
    "slider.css",
    "select.css",
    "radio.css",
    "text-input.css",
    "text.css",
);

theme!(
    NIGHT_THEME,
    "night" =>
    "default.css",
    "body.css",
    "button.css",
    "checkbox.css",
    "combobox.css",
    "div.css",
    "knob.css",
    "scroll.css",
    "slider.css",
    "select.css",
    "radio.css",
    "text-input.css",
    "text.css",
);
