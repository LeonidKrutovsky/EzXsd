use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{Fields, GenericArgument, ItemEnum, PathArguments, Type, TypePath};

pub trait Variant {
    fn path(&self) -> &TypePath;
    fn generic_type(&self) -> Option<&Ident>;
}

impl Variant for syn::Variant {
    fn path(&self) -> &TypePath {
        if let Fields::Unnamed(unnamed_fields) = &self.fields {
            if let Type::Path(type_path) = &unnamed_fields.unnamed[0].ty {
                let args = &type_path.path.segments[0].arguments;
                if let PathArguments::AngleBracketed(args) = args {
                    if let GenericArgument::Type(ty) =
                        args.args.first().expect("Generic arguments must exists")
                    {
                        if let Type::Path(type_path) = ty {
                            return type_path;
                        } else {
                            unreachable!("Not Path generic argument")
                        }
                    } else {
                        unreachable!("Generic Argument is not Type")
                    }
                } else if args == &PathArguments::None {
                    return type_path;
                } else {
                    return type_path;
                }
            } else {
                unreachable!("Not a TypePath in ty")
            }
        } else {
            unreachable!("Not an Unnamed fields in enum")
        }
    }

    fn generic_type(&self) -> Option<&Ident> {
        if let Fields::Unnamed(unnamed_fields) = &self.fields {
            if let Type::Path(type_path) = &unnamed_fields.unnamed[0].ty {
                if let PathArguments::AngleBracketed(_) = &type_path.path.segments[0].arguments {
                    return Some(&type_path.path.segments[0].ident);
                }
            }
        }
        None
    }
}

pub fn xsd_group(item: ItemEnum) -> TokenStream {
    let enum_name = &item.ident;
    let mut names = quote!();
    let mut match_cases = quote!();

    for var in &item.variants {
        let path = var.path();
        let name = quote! (
            #path::NAME,
        );
        names.extend(name);

        let case_name = &var.ident;
        let match_case = if let Some(gt) = var.generic_type() {
            quote! (
                #path::NAME => Self::#case_name(#gt::new(#path::parse(node)?)),
            )
        } else {
            quote! (#path::NAME => Self::#case_name(#path::parse(node)?),)
        };
        match_cases.extend(match_case);
    }

    let err_msg = format!("{} parsing error", enum_name);

    let output = quote! (
        #[derive(Debug)]
        #item

        impl #enum_name {
            pub const NAMES: &'static [&'static str] = &[
                #names
            ];

            pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
                Ok(match node.tag_name().name() {
                    #match_cases

                    _ => {return Err(format!("{}: {:?}", #err_msg, node))},
                })
            }
        }
    );

    output.into()
}
