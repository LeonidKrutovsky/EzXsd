use crate::named_argument::NamedArgument;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn xsd_group(arg: NamedArgument, item: ItemStruct) -> TokenStream {
    let group_name = arg.value;
    let struct_name = &item.ident;

    let output = quote! (
        #[derive(Debug, Default)]
        #item

        impl #struct_name {
            pub const NAME: &'static str = #group_name;
        }
    );

    output.into()
}
