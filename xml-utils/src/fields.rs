use proc_macro2::{Ident, TokenStream};
use syn::{GenericArgument, Path, PathArguments, Type};
use quote::quote;

#[derive(Debug)]
pub struct Field {
    name: Ident,
    type_name: Ident,
    type_scope: Option<Ident>,
}

impl Field {
    pub fn new(
        name: &Ident,
        type_name: &Ident,
        type_scope: Option<&Ident>,
    ) -> Self {
        Self{
            name: name.clone(),
            type_name: type_name.clone(),
            type_scope: type_scope.map(|t| t.clone())
        }
    }
    pub fn full_type(&self) -> TokenStream {
        let type_name = &self.type_name;
        if let Some(type_scope) = &self.type_scope {
            quote!(
                #type_scope::#type_name
            )
        } else {
            quote!(
                #type_name
            )
        }

    }
}


#[derive(Debug)]
pub enum FieldType {
    Option(Field),
    Vec(Field),
    Raw(Field),
}

impl FieldType {
    pub fn new(field: Field, generic_type: &Ident) -> Self {
        if generic_type == "Option" {
            Self::Option(field)
        } else if generic_type == "Vec" {
            Self::Vec(field)
        } else {
            Self::Raw(field)
        }
    }

    pub fn full_type(&self) -> TokenStream {
        let ty = match self {
            FieldType::Option(ref t) => t,
            FieldType::Vec(ref t) => t,
            FieldType::Raw(ref t) => t,
        }.full_type();

        match self {
            FieldType::Option(_) => quote! (Option<#ty>),
            FieldType::Vec(_) =>  quote! (Vec<#ty>),
            FieldType::Raw(_) =>  ty,
        }
    }

    pub fn field(&self) -> &Field {
        match self {
            FieldType::Option(ref t) => t,
            FieldType::Vec(ref t) => t,
            FieldType::Raw(ref t) => t,
        }
    }

    pub fn define(&self) -> TokenStream {
        let name = &self.field().name;
        quote! (
            let mut #name = None;
        )
    }

    pub fn match_row(&self) -> TokenStream {
        let name = &self.field().name;
        let ty = self.full_type();
        let field_type = self.field().full_type();
        quote! (
            #field_type::NAME => {#name = Some(#field_type::parse(ch)?)},
        )
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
    pub attributes: Vec<FieldType>,
    pub elements: Vec<FieldType>,
    pub groups: Vec<FieldType>,
    pub texts: Vec<FieldType>,
}

impl StructFields {
    pub fn add(&mut self, field: &syn::Field) {
        if let Type::Path(ty) = &field.ty {
            let type_path = &ty.path;
            let name = field.ident.as_ref().unwrap();
            let segment = type_path.segments.first().expect("Empty segments in Field");

            if &segment.ident == "Option" {
                let path = unpack_generic_argument(&segment.arguments).unwrap();
                self.push(path, &segment.ident, name);
            } else if &segment.ident == "Vec" {
                let path = unpack_generic_argument(&segment.arguments).unwrap();
                self.push(path, &segment.ident, name);
            } else {
                self.push(type_path, &segment.ident, name);
            }
        }
    }
    fn push(&mut self, ty: &Path, type_modifier: &Ident, name: &Ident) {
        if &ty.segments.len() > &1 {
            let type_scope = &ty.segments[0].ident;
            let field = Field::new(name, &ty.segments[1].ident, Some(type_scope));
            if type_scope == "attributes" {
                self.attributes.push(FieldType::new(field, type_modifier))
            } else if type_scope == "elements" {
                self.elements.push(FieldType::new(field, type_modifier))
            } else if type_scope == "groups" {
                self.groups.push(FieldType::new(field, type_modifier))
            }
        } else {
            let field = Field{
                name: name.clone(),
                type_name: ty.segments[0].ident.clone(),
                type_scope: None
            };
            self.texts.push(FieldType::new(field, type_modifier))
        }
    }
}
