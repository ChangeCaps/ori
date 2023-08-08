mod build;
mod font;
mod krate;
mod style;

/// Includes a font from a file, or directory.
#[manyhow::manyhow]
#[proc_macro]
pub fn font(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    font::font(input)
}

/// Includes a style sheet from a file, or directory.
///
/// This macro will load the style sheet from the given path if it exists, otherwise it will
/// include the style sheet as a string literal. The path is relative to the `CARGO_MANIFEST_DIR`.
#[manyhow::manyhow]
#[proc_macro]
pub fn style(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    style::style(input)
}

/// Macro for deriving the `Build` trait.
#[manyhow::manyhow]
#[proc_macro_derive(Build, attributes(prop, event, bind, children))]
pub fn derive_build(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    build::derive_build(input)
}
