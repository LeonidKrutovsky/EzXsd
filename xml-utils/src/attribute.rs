use crate::named_argument::NamedArgument;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct, Type};

pub fn xsd_attribute(arg: NamedArgument, item: ItemStruct) -> TokenStream {
    let attr_name = arg.value;
    let struct_name = &item.ident;

    let mut is_bool = false;
    if let Fields::Unnamed(field) = &item.fields {
        if let Type::Path(type_path) = &field.unnamed[0].ty {
            is_bool = type_path.path.segments[0].ident == "bool";
        }
    }

    let parse_body = if is_bool {
        quote! {Ok(attr.value().parse()?)}
    } else {
        quote! {Ok(Self(attr.value().parse()?))}
    };

    let from_str_body = if is_bool {
        quote! {
            if s == "0" || s == "false" {
                Ok(Self(false))
            } else if s == "1" || s == "true" {
                Ok(Self(true))
            } else {
                Err(format!("Attribute <{}> Error! Invalid value for boolean: {}", Self::NAME, s))
            }
        }
    } else {
        quote! {Ok(Self(s.parse()?))}
    };

    if is_bool {
        println!("{}", parse_body);
    }

    let output = quote! (
        #[derive(Debug, Default)]
        #item

        impl #struct_name {
            pub const NAME: &'static str = #attr_name;

            pub fn parse(attr: &roxmltree::Attribute) -> Result<Self, String> {
                #parse_body
            }

            pub fn text(&self) -> String {
                format!(" {}=\"{}\"", Self::NAME, self.0)
            }
        }

        impl std::str::FromStr for #struct_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #from_str_body
            }
        }

    );
    output.into()
}
