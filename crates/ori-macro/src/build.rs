use manyhow::bail;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Attribute, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident};

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
            if attr.path().is_ident("prop") {
                is_prop = true;
            } else if attr.path().is_ident("event") {
                is_event = true;
            } else if attr.path().is_ident("bind") {
                is_bind = true;
            } else if attr.path().is_ident("children") {
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

pub fn derive_build(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    let input = syn::parse::<DeriveInput>(input)?;

    let build = build(&input)?;
    let children = children(&input)?;

    let expanded = quote! {
        #build
        #children
    };

    Ok(expanded.into())
}

fn data(input: &DeriveInput) -> manyhow::Result<(&DataStruct, FieldsNamed)> {
    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => Ok((data, fields.clone())),
            Fields::Unnamed(_) => bail!(input, "tuple structs are not supported"),
            Fields::Unit => Ok((
                data,
                FieldsNamed {
                    brace_token: Default::default(),
                    named: Default::default(),
                },
            )),
        },
        Data::Enum(_) => bail!(input, "enum types are not supported"),
        Data::Union(_) => bail!(input, "union types are not supported"),
    }
}

fn build(input: &DeriveInput) -> manyhow::Result<TokenStream> {
    let name = &input.ident;

    let ori_core = find_crate("core");

    let property_setter = prop_setter(input)?;
    let event_setter = event_setter(input)?;
    let binding_setter = binding_setter(input)?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! { const _: () = {
        #[repr(transparent)]
        pub struct PropertiesSetter #impl_generics #where_clause {
            this: #name #ty_generics,
        }

        impl #impl_generics PropertiesSetter #ty_generics #where_clause {
            #property_setter
        }

        #[repr(transparent)]
        pub struct EventsSetter #impl_generics #where_clause {
            this: #name #ty_generics,
        }

        impl #impl_generics EventsSetter #ty_generics #where_clause {
            #event_setter
        }

        #[repr(transparent)]
        pub struct BindingsSetter #impl_generics #where_clause {
            this: #name #ty_generics,
        }

        impl #impl_generics BindingsSetter #ty_generics #where_clause {
            #binding_setter
        }

        impl #impl_generics #ori_core::Build for #name #ty_generics #where_clause {
            type Properties = PropertiesSetter #ty_generics;
            type Events = EventsSetter #ty_generics;
            type Bindings = BindingsSetter #ty_generics;

            fn build() -> #ori_core::View {
                #ori_core::View::new(Self::default())
            }

            fn prop_ref(&self) -> &<Self as #ori_core::Build>::Properties {
                // SAFETY: PropertiesSetter is repr(transparent) over Self.
                unsafe { ::std::mem::transmute(self) }
            }

            fn prop(&mut self) -> &mut <Self as #ori_core::Build>::Properties {
                // SAFETY: PropertiesSetter is repr(transparent) over Self.
                unsafe { ::std::mem::transmute(self) }
            }

            fn on_ref(&self) -> &<Self as #ori_core::Build>::Events {
                // SAFETY: EventsSetter is repr(transparent) over Self.
                unsafe { ::std::mem::transmute(self) }
            }

            fn on(&mut self) -> &mut <Self as #ori_core::Build>::Events {
                // SAFETY: EventsSetter is repr(transparent) over Self.
                unsafe { ::std::mem::transmute(self) }
            }

            fn bind_ref(&self) -> &<Self as #ori_core::Build>::Bindings {
                // SAFETY: BindingsSetter is repr(transparent) over Self.
                unsafe { ::std::mem::transmute(self) }
            }

            fn bind(&mut self) -> &mut <Self as #ori_core::Build>::Bindings {
                // SAFETY: BindingsSetter is repr(transparent) over Self.
                unsafe { ::std::mem::transmute(self) }
            }
        }
    };})
}

fn children(input: &DeriveInput) -> manyhow::Result<TokenStream> {
    let name = &input.ident;
    let (_, fields) = data(input)?;

    let ori_core = find_crate("core");

    let fields = fields.named.iter().filter_map(|field| {
        let field_name = &field.ident;
        let attrs = Attrs::parse(&field.attrs);

        if !attrs.is_children {
            return None;
        }

        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        Some(quote! {
            impl #impl_generics #ori_core::Parent for #name #ty_generics #where_clause {
                fn clear_children(&mut self) {
                    self.#field_name.clear_children();
                }

                fn add_children(
                    &mut self,
                    child: impl ::std::iter::Iterator<Item = #ori_core::View>,
                ) -> ::std::primitive::usize {
                    self.#field_name.add_children(child)
                }

                fn set_children(
                    &mut self,
                    slot: ::std::primitive::usize,
                    child: impl ::std::iter::Iterator<Item = #ori_core::View>,
                ) {
                    self.#field_name.set_children(slot, child);
                }
            }
        })
    });

    Ok(quote! {
        #(#fields)*
    })
}

fn prop_setter(input: &DeriveInput) -> manyhow::Result<TokenStream> {
    let (_, fields) = data(input)?;

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

    Ok(quote! {
        #(#fields)*
    })
}

fn event_name(name: &Ident) -> Ident {
    let name = name.to_string();
    let event_name = name.strip_prefix("on_").unwrap_or(&name).to_string();
    Ident::new(&event_name, name.span())
}

fn event_setter(input: &DeriveInput) -> manyhow::Result<TokenStream> {
    let (_, fields) = data(input)?;

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

    Ok(quote! {
        #(#fields)*
    })
}

fn binding_setter(input: &DeriveInput) -> manyhow::Result<TokenStream> {
    let (_, fields) = data(input)?;

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
                #name: #ori_reactive::Signal<<#ty as #ori_core::Bindable>::Item>
            ) {
                <#ty as #ori_core::Bindable>::bind(&mut self.this.#name, #name);
            }
        })
    });

    Ok(quote! {
        #(#fields)*
    })
}
