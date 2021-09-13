use crate::named_argument::NamedArgument;
use syn::ItemStruct;
use proc_macro::TokenStream;
use quote::quote;

pub fn xsd_group(arg: NamedArgument, item: ItemStruct) -> TokenStream {
    let group_name = arg.value;
    let struct_name = &item.ident;

    let output2 = quote! (
        pub const NAME2: &'static str = #group_name;
    );

    let mut output = quote! (
        #[derive(Debug, Default)]
        #item

        impl #struct_name {
            #output2
            pub const NAME: &'static str = #group_name;
        }
    );


    output.into()
}