use super::*;

#[derive(FromField)]
#[darling(attributes(back), forward_attrs(allow, doc, cfg))]
pub struct BackArgs {
    #[darling(default)]
    skip: bool,
}

pub fn derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let params = &input.generics.params;
    let unit = &input.ident;
    match &input.data {
        Data::Struct(data_struct) => {
            let mut backs = quote!{};
            for field in data_struct.fields.iter() {
                let args = match BackArgs::from_field(field) {
                    Ok(v) => v,
                    Err(e) => {
                        // If darling returned an error, generate a
                        // token stream from it so that the compiler
                        // shows the error in the right location.
                        return TokenStream::from(e.write_errors());
                    }
                };
                if !args.skip {
                    let name = field.ident.as_ref().unwrap();
                    backs.extend(quote! {
                        self.#name.back(back)?;
                    });
                }
            }
            if params.is_empty() {
                quote! {
                    #[automatically_derived]
                    impl graph::Adapt for #unit {
                        fn back(&mut self, back: &graph::Back) -> graph::Result<()> {
                            #backs
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
                            #backs
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


// let fields = struct_
//                 .fields
//                 .iter()
//                 .map(|item| item.ident.as_ref().unwrap())
//                 .collect::<Vec<_>>();

// #(
//     self.#fields.back(back)?;
// )*