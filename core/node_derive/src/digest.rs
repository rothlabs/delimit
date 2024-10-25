use super::*;

pub fn derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let unit = &input.ident;
    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let field_idents = fields
                .iter()
                .map(|item| item.ident.as_ref().unwrap())
                .collect::<Vec<_>>();
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
        }
        Data::Enum(syn::DataEnum { variants, .. }) => {
            let variant_idents = variants.iter().map(|item| &item.ident).collect::<Vec<_>>();
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
        }
        _ => unimplemented!(),
    }
    .into()
}
