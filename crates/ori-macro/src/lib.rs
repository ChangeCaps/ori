mod derive_style;
mod font;
mod krate;
mod react;

/// Include a font from a file, or directory.
#[manyhow::manyhow]
#[proc_macro]
pub fn font(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    font::font(input)
}

/// Include a style sheet from a file, or directory.
///
/// This macro will load the style sheet from the given path if it exists, otherwise it will
/// include the style sheet as a string literal. The path is relative to the `CARGO_MANIFEST_DIR`.
#[manyhow::manyhow]
#[proc_macro_derive(Style, attributes(name, state))]
pub fn derive_style(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    derive_style::derive_style(input)
}

/// Create a reactive node.
#[proc_macro]
pub fn react(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    react::react(input)
}
