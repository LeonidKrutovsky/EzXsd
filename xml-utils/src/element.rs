use proc_macro::TokenStream;

use quote::quote;
use syn::{Fields, ItemStruct};

use crate::fields::{StructFields};
use crate::named_argument::NamedArgument;

pub fn xsd_element(arg: NamedArgument, item: ItemStruct) -> TokenStream {
    let element_name = arg.value;
    let struct_name = &item.ident;
    let fields = &item.fields;

    let mut output = quote! (
        #[derive(Debug, Default)]
        #item


        impl #struct_name {
            pub const NAME: &'static str = #element_name;
        }
    );


    if struct_name == "Redefine" {
        let mut sf = StructFields::default();
        if let Fields::Named(ref fields_named) = fields {
            for field in &fields_named.named {

                sf.add(field);
            }
            println!("\n\n\n ************ \n\n\n ");
            // println!("{:#?}", sf);
            assert_eq!(sf.attributes.len(), 3);
            assert_eq!(sf.elements.len(), 1);
            assert_eq!(sf.groups.len(), 1);
            let attrs = sf.attributes[0].full_type().to_string();
            assert_eq!(attrs, "attributes :: AnyAttributes");
            let id = sf.attributes[2].full_type().to_string();
            assert_eq!(id, "Option < attributes :: Id >");
        }
        //println!("{:#?}", sf);

        let mut fields_define = quote! {};
        let mut fields_match = quote! {};
        for ref field_type in sf.elements {
            fields_define.extend(field_type.define());
            fields_match.extend(field_type.match_row());
        }

        let out = quote! (
            impl #struct_name {
                pub fn parse2(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
                    #fields_define
                    for ch in node.children().filter(|n| n.is_element()) {
                        match ch.tag_name().name() {
                            #fields_match
                            _ => {}
                        }
                    }
                    Err(String::new())
                }
            }
        );

        println!("{:#?}", out.to_string());
        output.extend(out);
    }



    let output2 = quote! (
        impl #struct_name {
            pub fn test() -> Option<#struct_name> {
                None
            }

            pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
                Err(String::new())
            }


        }
    );
    output.extend(output2);

    output.into()
}

