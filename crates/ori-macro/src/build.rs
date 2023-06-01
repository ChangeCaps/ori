use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Attribute, Data, DataStruct, DeriveInput,
    Fields, FieldsNamed, Generics, Ident,
};

use crate::krate::find_crate;

#[allow(dead_code)]
struct Attrs {
    is_prop: bool,
    is_event: bool,
    is_bind: bool,
    is_children: bool,
}

impl Attrs {
    pub fn parse(attrs: &[Attribute]) -> Self {
        let mut is_prop = false;
        let mut is_event = false;
        let mut is_bind = false;
        let mut is_children = false;

        for attr in attrs {
            if attr.path.is_ident("prop") {
                is_prop = true;
            } else if attr.path.is_ident("event") {
                is_event = true;
            } else if attr.path.is_ident("bind") {
                is_bind = true;
            } else if attr.path.is_ident("children") {
                is_children = true;
            }
        }

        Self {
            is_prop,
            is_event,
            is_bind,
            is_children,
        }
    }
}

pub fn derive_build(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let properties = properties(&input);
    let events = events(&input);
    let bindings = bindings(&input);
    let children = children(&input);

    let expanded = quote! {
        #properties
        #events
        #bindings
        #children
    };

    expanded.into()
}

fn data(input: &DeriveInput) -> (&DataStruct, &FieldsNamed) {
    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => (data, fields),
            Fields::Unnamed(_) => abort!(input, "tuple structs are not supported"),
            Fields::Unit => abort!(input, "unit structs are not supported"),
        },
        Data::Enum(_) => abort!(input, "enum types are not supported"),
        Data::Union(_) => abort!(input, "union types are not supported"),
    }
}

fn setter_generics(input: &DeriveInput) -> Generics {
    let mut generics = input.generics.clone();

    generics.params.push(parse_quote!('__setter));

    generics
}

fn properties(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let setter = prop_setter(input);

    let ori_core = find_crate("core");

    let setter_generics = setter_generics(input);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (setter_impl_generics, setter_ty_generics, setter_where_clause) =
        setter_generics.split_for_impl();

    quote! {
        const _: () = {
            pub struct Setter #setter_impl_generics #setter_where_clause {
                this: &'__setter mut #name #ty_generics,
            }

            impl #setter_impl_generics Setter #setter_ty_generics #setter_where_clause {
                #setter
            }

            impl #impl_generics #ori_core::Properties for #name #ty_generics #where_clause {
                type Setter<'__setter> = Setter #setter_ty_generics;

                fn setter(&mut self) -> Self::Setter<'_> {
                    Setter { this: self }
                }
            }
        };
    }
}

fn events(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let setter = event_setter(input);

    let ori_core = find_crate("core");

    let setter_generics = setter_generics(input);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (setter_impl_generics, setter_ty_generics, setter_where_clause) =
        setter_generics.split_for_impl();

    quote! {
        const _: () = {
            pub struct Setter #setter_impl_generics #setter_where_clause {
                this: &'__setter mut #name #ty_generics,
            }

            impl #setter_impl_generics Setter #setter_ty_generics #setter_where_clause {
                #setter
            }

            impl #impl_generics #ori_core::Events for #name #ty_generics #where_clause {
                type Setter<'__setter> = Setter #setter_ty_generics;

                fn setter(&mut self) -> Self::Setter<'_> {
                    Setter { this: self }
                }
            }
        };
    }
}

fn bindings(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let setter = binding_setter(input);

    let ori_core = find_crate("core");

    let setter_generics = setter_generics(input);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (setter_impl_generics, setter_ty_generics, setter_where_clause) =
        setter_generics.split_for_impl();

    quote! {
        const _: () = {
            pub struct Setter #setter_impl_generics #setter_where_clause {
                this: &'__setter mut #name #ty_generics,
            }

            impl #setter_impl_generics Setter #setter_ty_generics #setter_where_clause {
                #setter
            }

            impl #impl_generics #ori_core::Bindings for #name #ty_generics #where_clause {
                type Setter<'__setter> = Setter #setter_ty_generics;

                fn setter(&mut self) -> Self::Setter<'_> {
                    Setter { this: self }
                }
            }
        };
    }
}

fn children(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let (_, fields) = data(input);

    let ori_core = find_crate("core");

    let fields = fields.named.iter().filter_map(|field| {
        let field_name = &field.ident;
        let ty = &field.ty;

        let attrs = Attrs::parse(&field.attrs);

        if !attrs.is_children {
            return None;
        }

        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        Some(quote! {
            impl #impl_generics #ori_core::Parent for #name #ty_generics #where_clause {
                type Child = <#ty as #ori_core::Parent>::Child;

                fn clear_children(&mut self) {
                    self.#field_name.clear_children();
                }

                fn add_children(
                    &mut self,
                    child: impl ::std::iter::Iterator<Item = #ori_core::Node<Self::Child>>,
                ) -> ::std::primitive::usize {
                    self.#field_name.add_children(child)
                }

                fn set_children(
                    &mut self,
                    slot: ::std::primitive::usize,
                    child: impl ::std::iter::Iterator<Item = #ori_core::Node<Self::Child>>,
                ) {
                    self.#field_name.set_children(slot, child);
                }
            }
        })
    });

    quote! {
        #(#fields)*
    }
}

fn prop_setter(input: &DeriveInput) -> TokenStream {
    let (_, fields) = data(input);

    let fields = fields.named.iter().filter_map(|field| {
        let name = &field.ident;
        let ty = &field.ty;

        let attrs = Attrs::parse(&field.attrs);

        if !attrs.is_prop {
            return None;
        }

        Some(quote! {
            pub fn #name(&mut self, #name: impl ::std::convert::Into<#ty>) {
                self.this.#name = ::std::convert::Into::into(#name);
            }
        })
    });

    quote! {
        #(#fields)*
    }
}

fn event_name(name: &Ident) -> Ident {
    let name = name.to_string();
    let event_name = name.strip_prefix("on_").unwrap_or(&name).to_string();
    Ident::new(&event_name, name.span())
}

fn event_setter(input: &DeriveInput) -> TokenStream {
    let (_, fields) = data(input);

    let ori_core = find_crate("core");
    let ori_reactive = find_crate("reactive");

    let fields = fields.named.iter().filter_map(|field| {
        let name = field.ident.as_ref().unwrap();
        let event = event_name(name);

        let ty = &field.ty;

        let attrs = Attrs::parse(&field.attrs);

        if !attrs.is_event {
            return None;
        }

        Some(quote! {
            pub fn #event(
                &mut self,
                cx: #ori_reactive::Scope,
                #name: impl FnMut(&<#ty as #ori_core::BindCallback>::Event) + ::std::marker::Send + 'static
            ) {
                <#ty as #ori_core::BindCallback>::bind(&mut self.this.#name, cx, #name);
            }
        })
    });

    quote! {
        #(#fields)*
    }
}

fn binding_setter(input: &DeriveInput) -> TokenStream {
    let (_, fields) = data(input);

    let ori_core = find_crate("core");
    let ori_reactive = find_crate("reactive");

    let fields = fields.named.iter().filter_map(|field| {
        let name = field.ident.as_ref().unwrap();
        let event = event_name(name);

        let ty = &field.ty;

        let attrs = Attrs::parse(&field.attrs);

        if !attrs.is_bind {
            return None;
        }

        Some(quote! {
            pub fn #event(
                &mut self,
                cx: #ori_reactive::Scope,
                #name: #ori_reactive::Signal<<#ty as #ori_core::Bindable>::Item>
            ) {
                <#ty as #ori_core::Bindable>::bind(&mut self.this.#name, cx, #name);
            }
        })
    });

    quote! {
        #(#fields)*
    }
}
