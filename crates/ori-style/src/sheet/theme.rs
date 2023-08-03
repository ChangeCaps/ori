macro_rules! theme {
    ($name:ident, $folder:literal => $($style:literal),* $(,)?) => {
        #[allow(missing_docs)]
        pub const $name: &str = concat!(
            $(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/style/", $folder, "/", $style))),*
        );
    };
}

theme! {
    DEFAULT,
    "default" =>
    "default.css",
    "body.css",
    "button.css",
    "check-box.css",
    "combo-box.css",
    "div.css",
    "knob.css",
    "scroll.css",
    "slider.css",
    "select.css",
    "spacer.css",
    "radio.css",
    "text-input.css",
    "text.css",
}

theme! {
    DAY,
    "." =>
    "day.css",
}

theme! {
    NIGHT,
    "." =>
    "night.css",
}
