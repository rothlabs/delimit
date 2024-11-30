use super::*;

#[derive(FromField)]
#[darling(attributes(adapt), forward_attrs(allow, doc, cfg))]
pub struct AdaptArgs {
    #[darling(default)]
    skip: bool,
}

pub fn derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let unit = &input.ident;
    match &input.data {
        Data::Struct(struct_) => {
            // let fields = struct_
            //     .fields
            //     .iter()
            //     .map(|item| item.ident.as_ref().unwrap())
            //     .collect::<Vec<_>>();
            let mut adapts = quote! {};
            let mut backs = quote! {};
            for field in struct_.fields.iter() {
                let args = match AdaptArgs::from_field(field) {
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
                    adapts.extend(quote! {
                        self.#name.deal(stringify!(#name), deal)?;
                    });
                    backs.extend(quote! {
                        self.#name.back(back);
                    });
                }
            }
            quote! {
                #[automatically_derived]
                impl graph::Adapt for #unit {
                    fn adapt(&mut self, deal: &mut dyn graph::Deal) -> graph::Result<()> {
                        #adapts
                        Ok(())
                    }
                    fn back(&mut self, back: &graph::Back) {
                        #backs
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
//     self.#fields.deal(stringify!(#fields), deal)?;
// )*