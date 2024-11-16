use super::*;

pub fn derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let unit = &input.ident;
    let params = &input.generics.params;
    match &input.data {
        Data::Struct(_) => {
            if params.is_empty() {
                quote! {
                    #[automatically_derived]
                    impl graph::GateTag for #unit {}
                    // #[automatically_derived]
                    // impl #unit {
                    //     pub fn hub(self) -> graph::Result<graph::Hub<<#unit as graph::Solve>::Base>> {
                    //         Ok(self.gate()?.into())
                    //     }
                    // }
                }
            } else {
                quote! {
                    #[automatically_derived]
                    impl<T> graph::GateTag for #unit<T> {}
                    // impl<T> #unit<T>
                    // where
                    //     T: 'static + graph::Gather,
                    //     #unit<T>: graph::Solve,
                    //     <#unit<T> as graph::Solve>::Base: graph::Gather,
                    // {
                    //     pub fn hub(self) -> graph::Result<graph::Hub<<#unit<T> as graph::Solve>::Base>> {
                    //         Ok(self.gate()?.into())
                    //     }
                    // }
                }
            }
        }
        _ => unimplemented!(),
    }
    .into()
}
