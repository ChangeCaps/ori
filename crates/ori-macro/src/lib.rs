mod build;
mod krate;
mod view;

/// Macro for building a reactive view, with html-like syntax.
///
/// For more information see [examples](https://github.com/ChangeCaps/ori/tree/main/examples).
#[manyhow::manyhow]
#[proc_macro]
pub fn view(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    view::view(input)
}

/// Macro for deriving the `Build` trait.
#[manyhow::manyhow]
#[proc_macro_derive(Build, attributes(prop, event, bind, children))]
pub fn derive_build(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    build::derive_build(input)
}
