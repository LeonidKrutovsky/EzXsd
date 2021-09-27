mod attribute;
mod complex_type;
mod element;
mod field;
mod groups;
mod named_argument;
mod struct_fields;
mod utils;

extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::attribute::xsd_attribute;
use crate::complex_type::xsd_complex_type;
use crate::element::xsd_element;
use crate::groups::xsd_group;
use crate::named_argument::NamedArgument;


#[proc_macro_attribute]
pub fn attribute(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(_metadata as NamedArgument);
    let item = parse_macro_input!(input as syn::ItemStruct);

    xsd_attribute(args, item)
}

#[proc_macro_attribute]
pub fn element(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(_metadata as NamedArgument);
    let item = parse_macro_input!(input as syn::ItemStruct);
    xsd_element(args, item)
}

#[proc_macro_attribute]
pub fn test_attr(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    //let args = parse_macro_input!(_metadata as NamedArgument);
    let item = parse_macro_input!(input as syn::ItemStruct);
    println!("{:#?}", _metadata);
    quote!(#item).into()
    //xsd_element(args, item)
}

#[proc_macro_attribute]
pub fn group(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::ItemEnum);

    xsd_group(item)
}

#[proc_macro_attribute]
pub fn complex_type(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    //let args = parse_macro_input!(_metadata as NamedArgument);
    let item = parse_macro_input!(input as syn::ItemStruct);

    xsd_complex_type(item)
}
