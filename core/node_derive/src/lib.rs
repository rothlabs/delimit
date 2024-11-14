extern crate proc_macro2;

use darling::FromField;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Data;

mod adapt;
mod back;
mod digest;
mod gate;

#[proc_macro_derive(GateTag)]
pub fn gate_tag(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let unit = &input.ident;
    let params = &input.generics.params;
    match &input.data {
        Data::Struct(_) => {
            if params.is_empty() {
                quote! {
                    #[automatically_derived]
                    impl graph::GateTag for #unit {}
                }
            } else {
                quote! {
                    #[automatically_derived]
                    impl<T> graph::GateTag for #unit<T> {}
                }
            }
        }
        _ => unimplemented!(),
    }
    .into()
}

#[proc_macro_derive(Gate)]
pub fn gate(item: TokenStream) -> TokenStream {
    gate::derive(item)
}

#[proc_macro_derive(Adapt)]
pub fn adapt(item: TokenStream) -> TokenStream {
    adapt::derive(item)
}

#[proc_macro_derive(Back, attributes(back))]
pub fn back(item: TokenStream) -> TokenStream {
    back::derive(item)
}

#[proc_macro_derive(Digest)]
pub fn digest(item: TokenStream) -> TokenStream {
    digest::derive(item)
}
