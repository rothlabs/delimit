use super::*;

pub fn derive(item: TokenStream) -> TokenStream {
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
