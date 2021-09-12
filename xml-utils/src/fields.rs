use proc_macro2::Ident;
use syn::{Field, GenericArgument, Path, PathArguments, Type};

#[derive(Debug)]
pub enum StructField{
    Option(Ident),
    Vec(Ident),
    Raw(Ident)
}

impl StructField {
    pub fn new(path: &Path, ident: &Ident) -> Self {
        if ident == "Option" {
            Self::Option(path.segments[1].ident.clone())
        } else if ident == "Vec" {
            Self::Vec(path.segments[1].ident.clone())
        } else {
            Self::Raw(path.segments[1].ident.clone())
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
pub struct StructFields{
    pub attributes: Vec<StructField>,
    pub elements: Vec<StructField>,
    pub groups: Vec<StructField>,
}

impl StructFields {
    pub fn add(&mut self, field: &Field) {
        if let Type::Path(ty) = &field.ty {
            let path = &ty.path;
            let segment = path.segments.first().expect("Empty segments in Field");

            if &segment.ident == "Option" {
                let path = unpack_generic_argument(&segment.arguments).unwrap();
                self.push(path, &segment.ident);
            }
            else if &segment.ident == "Vec" {
                let path = unpack_generic_argument(&segment.arguments).unwrap();
                self.push(path, &segment.ident);
            } else {
                self.push(path, &segment.ident);
            }
        }
    }
    fn push(&mut self, path: &Path, ident: &Ident) {
        if &path.segments[0].ident == "attributes" {
            self.attributes.push(StructField::new(path, ident))
        } else if &path.segments[0].ident == "elements" {
            self.elements.push(StructField::new(path, ident))
        } else if &path.segments[0].ident == "groups" {
            self.groups.push(StructField::new(path, ident))
        }
    }
}