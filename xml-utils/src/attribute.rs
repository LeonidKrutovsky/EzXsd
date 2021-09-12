use crate::named_argument::NamedArgument;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn xsd_attribute(arg: NamedArgument, item: ItemStruct) -> TokenStream {
    let attr_name = arg.value;
    let struct_name = &item.ident;

    let output = quote! (
        #[derive(Debug, Default)]
        #item

        impl #struct_name {
            pub const NAME: &'static str = #attr_name;
        }

        impl std::convert::TryFrom<&roxmltree::Attribute<'_>> for #struct_name {
        type Error = String;

        fn try_from(attr: &roxmltree::Attribute) -> Result<Self, Self::Error> {
            Ok(Self(attr.value().parse()?))
            }
        }

        impl std::str::FromStr for #struct_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(s.parse()?))
            }
        }

    );
    output.into()
}
