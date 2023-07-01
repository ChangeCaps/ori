mod build;
mod krate;
mod view;

#[manyhow::manyhow]
#[proc_macro]
pub fn view(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    view::view(input)
}

#[manyhow::manyhow]
#[proc_macro_derive(Build, attributes(prop, event, bind, children))]
pub fn derive_build(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    build::derive_build(input)
}
