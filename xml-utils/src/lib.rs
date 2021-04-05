mod attribute;
mod named_argument;
mod element;

extern crate proc_macro;
extern crate syn;
use syn::{parse_macro_input};
use proc_macro::TokenStream;

use crate::attribute::xsd_attribute;
use crate::named_argument::NamedArgument;
use crate::element::xsd_element;

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
