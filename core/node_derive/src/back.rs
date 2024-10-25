use super::*;

pub fn derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let params = &input.generics.params;
    let unit = &input.ident;
    match &input.data {
        Data::Struct(struct_) => {
            let fields = struct_
                .fields
                .iter()
                .map(|item| item.ident.as_ref().unwrap())
                .collect::<Vec<_>>();
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
        }
        _ => unimplemented!(),
    }
    .into()
}
