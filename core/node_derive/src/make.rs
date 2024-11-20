use super::*;

pub fn derive(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let unit = &input.ident;
    let builder = format_ident!("{}Builder", unit);
    let unit = syn::Ident::new(&unit.to_string().to_lowercase(), unit.span());
    match &input.data {
        Data::Struct(_) => {
            quote! {
                pub fn #unit() -> #builder {
                    #builder::default()
                }
            }
        },
        _ => unimplemented!()
    }.into()
}