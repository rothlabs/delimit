extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(Adapt)]
pub fn adapt_derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let struct_identifier = &input.ident;
    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let field_idents = fields.iter().map(|item| item.ident.as_ref().unwrap()).collect::<Vec<_>>();
            quote! {
                #[automatically_derived]
                impl Adapt for #struct_identifier {
                    fn adapt(&mut self, deal: &mut dyn Deal) -> graph::Result<()> {
                        #(
                            self.#field_idents.deal(stringify!(#field_idents), deal)?;
                        )*
                        Ok(())
                    }
                }
            }
        },
        _ => unimplemented!()
    }.into()
}

#[proc_macro_derive(Digest)]
pub fn digest_derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let struct_identifier = &input.ident;
    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let field_idents = fields.iter().map(|item| item.ident.as_ref().unwrap()).collect::<Vec<_>>();
            quote! {
                #[automatically_derived]
                impl Digest for #struct_identifier {
                    fn digest<H: std::hash::Hasher>(&self, state: &mut H) {
                        #(
                            self.#field_idents.digest(state);
                        )*
                    }
                }
            }
        },
        Data::Enum(syn::DataEnum {variants, ..}) => {
            let variant_idents = variants.iter().map(|item| &item.ident ).collect::<Vec<_>>();
            // let variant_fields = variants.iter().map(|item| &item.fields ).collect::<Vec<_>>();
            quote! {
                #[automatically_derived]
                impl Digest for #struct_identifier {
                    fn digest<H: std::hash::Hasher>(&self, state: &mut H) {
                        match self {
                            #(
                                Self::#variant_idents(x) => x.digest(state),
                            )*
                        }
                    }
                }
            }
        },
        _ => unimplemented!()
    }.into()
}