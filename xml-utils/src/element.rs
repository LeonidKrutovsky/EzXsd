use syn::{ItemStruct, Fields, Type, PathArguments, GenericArgument};
use proc_macro::TokenStream;
use quote::quote;
use crate::named_argument::NamedArgument;

pub fn xsd_element(arg: NamedArgument, item: ItemStruct) -> TokenStream {
    let element_name = arg.value;
    let struct_name = &item.ident;
    let fields = &item.fields;
    let output = quote! (
        #[derive(Debug, Default)]
        #item

        impl #struct_name {
            pub const NAME: &'static str = #element_name;
        }



    );
    if struct_name == "Redefine" {
        if let Fields::Named(ref hui) = fields {
            let named = &hui.named;
            for field in named {
                println!("{:?}", field.ident.as_ref().unwrap());

                if let Type::Path(ty) = &field.ty {
                    let path = &ty.path;
                    let segment = path.segments.first().unwrap();
                    if &segment.ident == "attributes" {
                        println!("Attribute: {:#?}", &path.segments.last().unwrap().ident);
                    }
                    else if  &segment.ident == "elements" {
                        println!("Element: {:#?}", &path.segments.last().unwrap().ident);
                    }
                    else if  &segment.ident == "Option" {
                        println!("Option Args: {:#?}", segment);
                    }
                    else if  &segment.ident == "Vec" {
                        if let PathArguments::AngleBracketed(zlp) = &segment.arguments {
                            if let GenericArgument::Type(ty) = zlp.args.first().unwrap() {
                                if let Type::Path(type_path) = ty {
                                    let path = &type_path.path;
                                    println!("Vector Args: {:#?}", path);
                                }

                            } else {
                                unreachable!()
                            }
                        }

                    }
                    for segment in &path.segments {


                    }
                }
                println!("\n\n\n ************ \n\n\n ");
            }

        }

    }

    output.into()
}