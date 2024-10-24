extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::Data;

#[proc_macro_derive(Gate)]
pub fn gate_derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let unit = &input.ident;
    let params = &input.generics.params;
    let builder = format_ident!("{}Builder", unit);
    match &input.data {
        Data::Struct(_) => {
            if params.is_empty() {
                quote! {
                    #[automatically_derived]
                    impl graph::GateTag for #unit {}
                    #[automatically_derived]
                    impl #builder {
                        pub fn make(self) -> graph::Result<#unit> {
                            match self.build() {
                                Ok(value) => Ok(value),
                                Err(err) => Err(anyhow!(err.to_string()))?,
                            }
                        }
                        pub fn node(self) -> graph::Result<graph::Node<#unit>> {
                            self.make()?.node()
                        }
                        pub fn hub(self) -> graph::Result<graph::Hub<<#unit as graph::Solve>::Base>> {
                            Ok(self.make()?.gate()?.into())
                        }
                    }
                }
            } else {
                quote! {
                    #[automatically_derived]
                    impl<T> graph::GateTag for #unit<T> {}
                    #[automatically_derived]
                    impl<T> #builder<T>
                    where
                        T: 'static + Clone + graph::SendSync + std::fmt::Debug,
                        #unit<T>: graph::Solve,
                        <#unit<T> as graph::Solve>::Base: Clone + graph::SendSync + std::fmt::Debug,
                    {
                        pub fn make(self) -> graph::Result<#unit<T>> {
                            match self.build() {
                                Ok(value) => Ok(value),
                                Err(err) => Err(anyhow!(err.to_string()))?,
                            }
                        }
                        pub fn node(self) -> graph::Result<graph::Node<#unit<T>>> {
                            self.make()?.node()
                        }
                        pub fn hub(self) -> graph::Result<graph::Hub<<#unit<T> as graph::Solve>::Base>> {
                            Ok(self.make()?.gate()?.into())
                        }
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
    let unit = &input.ident;
    match &input.data {
        Data::Struct(struct_) => {
            let fields = struct_.fields.iter().map(|item| item.ident.as_ref().unwrap()).collect::<Vec<_>>();
            quote! {
                #[automatically_derived]
                impl graph::Adapt for #unit {
                    fn adapt(&mut self, deal: &mut dyn graph::Deal) -> graph::Result<()> {
                        #(
                            self.#fields.deal(stringify!(#fields), deal)?;
                        )*
                        Ok(())
                    }
                }
            }
        },
        _ => unimplemented!()
    }.into()
}

#[proc_macro_derive(Back)]
pub fn back_derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let params = &input.generics.params;
    let unit = &input.ident;
    match &input.data {
        Data::Struct(struct_) => {
            let fields = struct_.fields.iter().map(|item| item.ident.as_ref().unwrap()).collect::<Vec<_>>();
            if params.is_empty() {
                quote! {
                    #[automatically_derived]
                    impl graph::Adapt for #unit {
                        fn back(&mut self, back: &graph::Back) -> graph::Result<()> {
                            #(
                                self.#fields.back(back)?;
                            )*
                            Ok(())
                        }
                    }
                }
            } else {
                quote! {
                    #[automatically_derived]
                    impl<T> graph::Adapt for #unit<T>
                    where
                        T: 'static + Clone + graph::SendSync, 
                    {
                        fn back(&mut self, back: &graph::Back) -> graph::Result<()> {
                            #(
                                self.#fields.back(back)?;
                            )*
                            Ok(())
                        }
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
    let unit = &input.ident;
    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let field_idents = fields.iter().map(|item| item.ident.as_ref().unwrap()).collect::<Vec<_>>();
            quote! {
                #[automatically_derived]
                impl graph::Digest for #unit {
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
                impl graph::Digest for #unit {
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