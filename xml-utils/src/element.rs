use proc_macro2::{Ident, TokenStream};

use quote::quote;
use syn::{Fields, ItemStruct};
use crate::field::FieldWrapper;

use crate::named_argument::NamedArgument;
use crate::struct_fields::StructFields;

fn assert_redefine(sf: &StructFields) {
    assert_eq!(sf.attributes.len(), 2);
    assert_eq!(sf.elements.len(), 1);
    assert_eq!(sf.groups.len(), 1);
    let attrs = sf.attributes[0].full_type().to_string();
    assert_eq!(attrs, "attributes :: SchemaLocation");
    let id = sf.attributes[1].full_type().to_string();
    assert_eq!(id, "attributes :: Id");
}

fn parse_struct(sf: &StructFields, struct_name: &Ident) -> TokenStream {
    let fields_define = sf.define_fields();
    let fields_match = sf.match_elements();
    let attributes_match = sf.match_attributes();
    let assign_lines = sf.assign_lines();

    let result = quote! (
        impl #struct_name {
            pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
                #fields_define
                #fields_match
                #attributes_match
                Ok(Self{
                    #assign_lines
                })
            }
        }
    );

    //println!("{}", result);
    result
}

pub fn xsd_element(arg: NamedArgument, item: ItemStruct) -> proc_macro::TokenStream {
    let element_name = arg.value;
    let struct_name = &item.ident;
    let fields = &item.fields;

    let mut output = quote! (
        #[derive(Debug)]
        #item
        impl #struct_name {
            pub const NAME: &'static str = #element_name;
        }
    );

    let mut sf = StructFields::default();
    if let Fields::Named(ref fields_named) = fields {
        for field in &fields_named.named {

            if struct_name == "TopLevelElement" {
                 //println!("{:#?} \n", field);
                println!("Name = {}",field.name());
                let tp = field.full_type();
                println!("FullType = {}",quote!{#tp});
                let tp = field.type_name();
                println!("TypeName = {}", tp);
            }
             sf.add(field);
        }
    }

    if struct_name == "Redefine" {
        assert_redefine(&sf);
    }

    let documentation = parse_struct(&sf, struct_name);
        output.extend(documentation);

    output.into()
}

pub fn xsd_element_test(arg: NamedArgument, item: ItemStruct) -> proc_macro::TokenStream {
    let element_name = arg.value;
    let struct_name = &item.ident;
    let fields = &item.fields;

    let mut output = quote! ();




    let mut sf = StructFields::default();
    if let Fields::Named(ref fields_named) = fields {
        let mut q = quote! {};
        for field in &fields_named.named {
            let name = field.name();
            let tp = field.full_type();
            q.extend(quote!(pub #name: #tp,));
            if name == "nillable" {
                println!("{:#?}", field);
            }

             sf.add(field);
        }
        println!("{}", q);
        let s = quote! (
            #[derive(Debug)]
        pub struct #struct_name {
                #q
            }


        impl #struct_name {
            pub const NAME: &'static str = #element_name;
        }
    );
        output.extend(s);
    }

    let documentation = parse_struct(&sf, struct_name);
        output.extend(documentation);

    output.into()
}
