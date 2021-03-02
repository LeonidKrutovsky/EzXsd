extern crate proc_macro;
extern crate syn;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;
//use syn::spanned::Spanned;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed, Item};


#[proc_macro_attribute]
pub fn attribute(_metadata: TokenStream, input: TokenStream) -> TokenStream {


    for i in _metadata {
        match i {
        TokenTree::Literal(ref lit) => {

            println!("***********{:#?}", lit);
        },
        _ => unreachable!()
    }
    }
    //let z: syn::Meta =  syn::parse(_metadata).expect("failed to parse input");

    let item: syn::Item = syn::parse(input).expect("failed to parse input");
    println!("***********{:#?}", item);
    match item {
        Item::Struct(ref struct_item) => {
            println!("{:?}", struct_item);
        }
        _ => {
            unreachable!()
        }
    }

    let output = quote! { #item };
    output.into()
}

#[proc_macro_derive(Attribute)]
pub fn same_name(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let description = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let idents = named.iter().map(|f| &f.ident);
                format!(
                    "a struct with these named fields: {}",
                    quote! {#(#idents), *}
                )
            }
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let num_fields = unnamed.iter().count();
                format!("a struct with {} unnamed fields", num_fields)
            }
            syn::Fields::Unit => format!("a unit struct"),
        },
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let vs = variants.iter().map(|v| &v.ident);
            format!("an enum with these variants: {}", quote! {#(#vs),*})
        }
        syn::Data::Union(DataUnion {
            fields: FieldsNamed { named, .. },
            ..
        }) => {
            let idents = named.iter().map(|f| &f.ident);
            format!("a union with these named fields: {}", quote! {#(#idents),*})
        }
    };

    let output = quote! {
    impl #ident {
        fn describe() {
        println!("{} is {}.", stringify!(#ident), #description);
        }
    }
    };

    output.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
