use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    parse::ParseStream, punctuated::Punctuated, spanned::Spanned, DeriveInput, Expr, ExprLit,
    Field, FieldsNamed, Lit, LitStr, Token,
};

use crate::krate::find_crate;

struct Attributes {
    name: String,
    states: Vec<String>,
}

impl Attributes {
    fn new(name: String, attrs: &[syn::Attribute]) -> manyhow::Result<Attributes> {
        let mut this = Attributes {
            name,
            states: Vec::new(),
        };

        for attr in attrs {
            if attr.path().is_ident("name") {
                let meta = attr.meta.require_name_value()?;
                let name = match meta.value {
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(ref lit_str),
                        ..
                    }) => lit_str.value(),
                    _ => manyhow::bail!(meta.value, "expected string literal"),
                };

                this.name = name;
            }

            if attr.path().is_ident("state") {
                let states = attr.parse_args_with(|parser: ParseStream| {
                    Punctuated::<LitStr, Token![,]>::parse_separated_nonempty(parser)
                })?;

                this.states.extend(states.iter().map(LitStr::value));
            }
        }

        Ok(this)
    }
}

pub fn derive_style(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    let input = syn::parse::<DeriveInput>(input)?;
    let name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let attrs = Attributes::new(name.to_string(), &input.attrs)?;
    let default_style = style_fn(&input, &attrs.name, None)?;

    let mut state_fns = Vec::new();

    for state in &attrs.states {
        state_fns.push(style_fn(&input, &attrs.name, Some(state))?);
    }

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics #name #ty_generics #where_clause {
            #default_style

            #(#state_fns)*
        }
    };

    Ok(expanded.into())
}

fn style_fn(input: &DeriveInput, name: &str, state: Option<&str>) -> manyhow::Result<TokenStream> {
    match input.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => style_fn_inner(fields, name, state),
            _ => manyhow::bail!("`#[derive(Style)]` can only be used on structs with named fields"),
        },
        _ => manyhow::bail!("`#[derive(Style)]` can only be used on structs"),
    }
}

fn style_fn_inner(
    fields: &FieldsNamed,
    name: &str,
    state: Option<&str>,
) -> manyhow::Result<TokenStream> {
    let ori_style = find_crate("style");

    let fields = fields
        .named
        .iter()
        .map(|field| style_attr_field(name, field, state))
        .collect::<Vec<_>>();

    let ident = match state {
        Some(state) => Ident::new(state, Span::call_site()),
        None => Ident::new("style", Span::call_site()),
    };

    Ok(quote! {
        pub fn #ident(theme: &#ori_style::Theme) -> Self {
            Self {
                #(#fields),*
            }
        }
    })
}

fn style_attr_field(name: &str, field: &Field, state: Option<&str>) -> TokenStream {
    let field_name = &field.ident;
    let value = style_attr_value(name, field, state);

    quote_spanned! {field.span()=>
        #field_name: #value
    }
}

fn style_attr_value(name: &str, field: &Field, state: Option<&str>) -> TokenStream {
    let ori_style = find_crate("style");

    let attr_name = style_attr_name(name, field, state);

    if state.is_some() {
        let fallback = style_attr_value(name, field, None);

        quote_spanned! {field.span()=>
            match theme.try_get(#ori_style::Attr::new(#attr_name)) {
                ::std::option::Option::Some(value) => value,
                ::std::option::Option::None => #fallback,
            }
        }
    } else {
        quote_spanned! {field.span()=>
            theme.get(#ori_style::Attr::new(#attr_name))
        }
    }
}

fn snake_to_kebab(snake: &str) -> String {
    snake.replace('_', "-")
}

fn style_attr_name(name: &str, field: &Field, state: Option<&str>) -> String {
    let field_name = snake_to_kebab(&field.ident.as_ref().unwrap().to_string());

    match state {
        Some(state) => {
            format!("{}[{}].{}", name, state, field_name)
        }
        None => {
            format!("{}.{}", name, field_name)
        }
    }
}
