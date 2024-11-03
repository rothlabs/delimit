extern crate proc_macro2;

use darling::FromField;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Data;

mod adapt;
mod back;
mod digest;
mod gate;

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
