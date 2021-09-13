use proc_macro::TokenStream;

use quote::quote;
use syn::{Fields, ItemStruct};

use crate::fields::StructFields;
use crate::named_argument::NamedArgument;

pub fn xsd_element(arg: NamedArgument, item: ItemStruct) -> TokenStream {
    let element_name = arg.value;
    let struct_name = &item.ident;
    let fields = &item.fields;

    let output2 = quote! (
        pub const NAME2: &'static str = #element_name;
    );

    let mut output = quote! (
        #[derive(Debug, Default)]
        #item

        impl #struct_name {
            #output2
            pub const NAME: &'static str = #element_name;

            pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
                Err("empty".to_string())
            }
        }
    );

    let output2 = quote! (
        impl #struct_name {
            pub fn test() -> Option<#struct_name> {
                None
            }
        }
    );
    output.extend(output2);

    if struct_name == "Redefine" {
        let mut sf = StructFields::default();
        if let Fields::Named(ref fields_named) = fields {
            for field in &fields_named.named {
                sf.add(field);
            }
            println!("\n\n\n ************ \n\n\n ");
            println!("{:#?}", sf);
            assert_eq!(sf.attributes.len(), 2);
            assert!(sf.any_attributes_allowed);
            assert_eq!(sf.elements.len(), 1);
            assert_eq!(sf.groups.len(), 1);
        }
    }

    output.into()
}
