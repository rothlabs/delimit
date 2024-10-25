use super::*;

pub fn derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let unit = &input.ident;
    match &input.data {
        Data::Struct(struct_) => {
            let fields = struct_
                .fields
                .iter()
                .map(|item| item.ident.as_ref().unwrap())
                .collect::<Vec<_>>();
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
        }
        _ => unimplemented!(),
    }
    .into()
}
