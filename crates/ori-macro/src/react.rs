use proc_macro2::TokenStream;
use quote::quote;

use crate::krate::find_crate;

pub fn react(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let ori_core = find_crate("core");

    let expanded = quote! {
        #ori_core::function::react(cx, move |cx| { #input })
    };

    expanded.into()
}
