extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::Data;

#[proc_macro_derive(Gate)]
pub fn gate_derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let unit = &input.ident;
    let builder = format_ident!("{}Builder", unit);
    match &input.data {
        Data::Struct(_) => {
            quote! {
                #[automatically_derived]
                impl GateTag for #unit {}
                #[automatically_derived]
                impl #builder 
                where 
                    #unit: Solve
                {
                    pub fn make(self) -> graph::Result<#unit> {
                        match self.build() {
                            Ok(value) => Ok(value),
                            Err(err) => Err(anyhow!(err.to_string()))?,
                        }
                    }
                    pub fn node(self) -> graph::Result<Node<#unit>> {
                        self.make()?.node()
                    }
                    pub fn hub(self) -> graph::Result<Hub<<#unit as Solve>::Base>> {
                        Ok(self.make()?.gate()?.into())
                    }
                }
            }
        },
        _ => unimplemented!()
    }.into()
}

#[proc_macro_derive(Reader)]
pub fn reader_derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let unit = &input.ident;
    let builder = format_ident!("{}Builder", unit);
    match &input.data {
        Data::Struct(_) => {
            quote! {
                #[automatically_derived]
                impl<T> GateTag for #unit<T> {}
                #[automatically_derived]
                impl<T: AnyBitPattern> #builder<T>
                where
                    #unit<T>: Solve
                {
                    pub fn make(self) -> graph::Result<#unit<T>> {
                        match self.build() {
                            Ok(value) => Ok(value),
                            Err(err) => Err(anyhow!(err.to_string()))?,
                        }
                    }
                    pub fn node(self) -> graph::Result<Node<#unit<T>>> {
                        self.make()?.node()
                    }
                    pub fn hub(self) -> graph::Result<Hub<<#unit<T> as Solve>::Base>> {
                        Ok(self.make()?.gate()?.into())
                    }
                }
            }
        },
        _ => unimplemented!()
    }.into()
}

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