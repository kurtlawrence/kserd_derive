//! [![Build Status](https://travis-ci.com/kurtlawrence/kserd_derive.svg?branch=master)](https://travis-ci.com/kurtlawrence/kserd_derive)
//! [![Latest Version](https://img.shields.io/crates/v/kserd_derive.svg)](https://crates.io/crates/kserd_derive)
//! [![Rust
//! Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/kserd_derive)
//! [![codecov](https://codecov.io/gh/kurtlawrence/kserd_derive/branch/master/graph/badge.svg)](https://codecov.io/gh/kurtlawrence/kserd_derive)
//!
//! Proc macro derive for **K**urt's **S**elf **E**xplanatory **R**ust **D**ata.
//!
//! See the [rs docs.](https://docs.rs/kserd_derive/)
//! Look at progress and contribute on [github.](https://github.com/kurtlawrence/kserd_derive)
//!
//! The main source of information is at [kurtlawrence/kserd](https://github.com/kurtlawrence/kserd).
//!
//! ## IMPORTANT - WIP
//!
//! `kserd_derive` is a work in progress and not currently usable but is actively being developed.
extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Ident, Index,
};

#[proc_macro_derive(AsKserd)]
pub fn derive_to_kserd(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: AsKserd` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression call as_kserd() of each field.
    let kserd_expr = to_kserd_expr(&input.data, &name);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics kserd::AsKserd for #name #ty_generics #where_clause {
            fn as_kserd<'a>(&'a self) -> kserd::Kserd<'a> {
                #kserd_expr
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

/// Add a bound `T: AsKserd` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(kserd::AsKserd));
        }
    }
    generics
}

/// Generate an expression call as_kserd() of each field.
fn to_kserd_expr(data: &Data, input_name: &Ident) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let names = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote! { stringify!(#name).into() }
                    });

                    let values = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! { f.span() =>
                            self.#name.as_kserd()
                        }
                    });

                    let tokens = quote! {
                        let mut map: BTreeMap<kserd::KserdStr<'a>, kserd::Kserd<'a>> = BTreeMap::new();
                        #(map.insert(#names, #values);)*
                        kserd::Kserd::with_identity(stringify!(#input_name), Value::Cntr(map))
                    };

                    tokens
                }
                Fields::Unnamed(ref fields) => {
                    unimplemented!();
                    // // Expands to an expression like
                    // //
                    // //     0 + self.0.heap_size() + self.1.heap_size() + self.2.heap_size()
                    // let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    //     let index = Index::from(i);
                    //     quote_spanned! {f.span()=>
                    //         heapsize::HeapSize::heap_size_of_children(&self.#index)
                    //     }
                    // });
                    // quote! {
                    //     0 #(+ #recurse)*
                    // }
                }
                Fields::Unit => {
                    unimplemented!();
                    //                     quote! {
                    //                         kserd::Kserd::with_identity(stringify!(#input_name), Value::Unit)
                    //                     }
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
