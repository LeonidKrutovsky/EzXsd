use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;
//use crate::named_argument::NamedArgument;

pub fn xsd_complex_type(item: ItemStruct) -> TokenStream {
    let output = quote! (
        #[derive(Debug, Default)]
        #item
    );

    //println!("{:#?}", item);
    output.into()
}
