use proc_macro2::TokenStream;
use quote::quote;

use crate::krate::find_crate;

pub fn reactive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let ori_core = find_crate("core");

    let expanded = quote! {
        #ori_core::function::reactive(cx, move |cx| { #input })
    };

    expanded.into()
}
