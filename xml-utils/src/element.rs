use quote::quote;
use syn::{Fields, ItemStruct};

use crate::field::{FieldWrapper, NamedFields};
use crate::named_argument::NamedArgument;

pub fn xsd_element(arg: NamedArgument, item: ItemStruct) -> proc_macro::TokenStream {
    let element_name = arg.value;
    let struct_name = &item.ident;
    let fields = &item.fields;

    let mut output = quote!();

    if let Fields::Named(fields_named) = fields {
        let mut fields_stream = quote! {};
        for field in &fields_named.named {
            let tp = field.full_type();
            let name = field.name();
            fields_stream.extend(quote! {pub #name: #tp,})
        }
        let parse_method = fields_named.impl_parse(&element_name);
        let text_method = fields_named.impl_text(&element_name);
        output = quote! (
            #[derive(Debug)]
            pub struct #struct_name {
                #fields_stream
            }
            impl #struct_name {
                pub const NAME: &'static str = #element_name;
                #parse_method

                #text_method
            }
        );
    }
    output.into()
}
