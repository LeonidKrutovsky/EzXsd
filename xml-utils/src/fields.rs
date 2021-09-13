use proc_macro2::Ident;
use syn::{Field, GenericArgument, Path, PathArguments, Type};
use quote::quote;
use proc_macro::TokenStream;

#[derive(Debug)]
pub enum StructField {
    Option(Ident),
    Vec(Ident),
    Raw(Ident),
}

impl StructField {
    pub fn new(first_ident: &Ident, generic_wrapper: &Ident) -> Self {
        if generic_wrapper == "Option" {
            Self::Option(first_ident.clone())
        } else if generic_wrapper == "Vec" {
            Self::Vec(first_ident.clone())
        } else {
            Self::Raw(first_ident.clone())
        }
    }

    pub fn to_token_stream(&self) -> TokenStream {
        match self {
            StructField::Option(i) => quote!(),
            StructField::Vec(i) => quote!(),
            StructField::Raw(i) => quote!(),
        }.into()
    }
}

fn unpack_generic_argument(arguments: &PathArguments) -> Option<&Path> {
    if let PathArguments::AngleBracketed(zlp) = arguments {
        if let GenericArgument::Type(ty) = zlp.args.first().unwrap() {
            if let Type::Path(type_path) = ty {
                Some(&type_path.path)
            } else {
                unreachable!("Not Path generic argument")
            }
        } else {
            unreachable!()
        }
    } else {
        None
    }
}

#[derive(Default, Debug)]
pub struct StructFields {
    pub attributes: Vec<StructField>,
    pub elements: Vec<StructField>,
    pub groups: Vec<StructField>,
    pub texts: Vec<StructField>,
    pub any_attributes_allowed: bool,
    pub any_elements_allowed: bool,
}

impl StructFields {
    pub fn add(&mut self, field: &Field) {

        if let Type::Path(ty) = &field.ty {
            let path = &ty.path;
            let segment = path.segments.first().expect("Empty segments in Field");

            if &segment.ident == "Option" {
                let path = unpack_generic_argument(&segment.arguments).unwrap();
                self.push(path, &segment.ident);
            } else if &segment.ident == "Vec" {
                let path = unpack_generic_argument(&segment.arguments).unwrap();
                self.push(path, &segment.ident);
            } else {
                let segment_ident = &segment.ident;
                print!("SEGMANT IDENT     {:#?}", segment_ident);
                self.push(path, &segment.ident);
            }
        }
    }
    fn push(&mut self, path: &Path, ident: &Ident) {
        let namespace_ = &path.segments[0].ident;
        let type_name = &path.segments[1].ident;
        if namespace_ == "attributes" {
            if type_name == "AnyAttributes" {
                self.any_attributes_allowed = true;
            } else {
                self.attributes.push(StructField::new(type_name, ident))
            }
        } else if namespace_ == "elements" {
            if type_name == "AnyElements" {
                self.any_elements_allowed = true;
            } else {
                self.elements.push(StructField::new(type_name, ident))
            }

        } else if namespace_ == "groups" {
            self.groups
                .push(StructField::new(&path.segments[1].ident, ident))
        } else if namespace_ == "String" {
            self.texts.push(StructField::new(namespace_, ident))
        }
    }

    pub fn to_token_stream(&self) -> TokenStream {
        quote! (
            for ch in node.children().filter(|n| n.is_element()) {

            }
        ).into()
    }
}
