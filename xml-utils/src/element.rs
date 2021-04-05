use syn::ItemStruct;
use proc_macro::TokenStream;
use quote::quote;
use crate::named_argument::NamedArgument;

pub fn xsd_element(arg: NamedArgument, item: ItemStruct) -> TokenStream {
    let element_name = arg.value;
    let struct_name = &item.ident;

    let output = quote! (
        #[derive(Debug, Default)]
        #item

        impl #struct_name {
            pub const NAME: &'static str = #element_name;
        }



    );

    //println!("{:#?}", item);
    output.into()
}