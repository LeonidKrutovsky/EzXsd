use proc_macro2::Ident;
use syn::{Field, GenericArgument, Path, PathArguments, Type};

#[derive(Debug)]
pub enum StructField {
    Option(Ident),
    Vec(Ident),
    Raw(Ident),
}

impl StructField {
    pub fn new(first_ident: &Ident, second_ident: &Ident) -> Self {
        if second_ident == "Option" {
            Self::Option(first_ident.clone())
        } else if second_ident == "Vec" {
            Self::Vec(first_ident.clone())
        } else {
            Self::Raw(first_ident.clone())
        }
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
                self.push(path, &segment.ident);
            }
        }
    }
    fn push(&mut self, path: &Path, ident: &Ident) {
        let first_ident = &path.segments[0].ident;
        if first_ident == "attributes" {
            self.attributes
                .push(StructField::new(&path.segments[1].ident, ident))
        } else if first_ident == "elements" {
            self.elements
                .push(StructField::new(&path.segments[1].ident, ident))
        } else if first_ident == "groups" {
            self.groups
                .push(StructField::new(&path.segments[1].ident, ident))
        } else if first_ident == "String" {
            self.texts.push(StructField::new(first_ident, ident))
        }
    }
}
