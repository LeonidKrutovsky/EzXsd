use crate::field::Field;
use crate::utils::unpack_generic_argument;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Path, Type};

#[derive(Default, Debug)]
pub struct StructFields {
    pub attributes: Vec<Field>,
    pub elements: Vec<Field>,
    pub groups: Vec<Field>,
    pub texts: Vec<Field>,
    pub any_elements: Option<Field>,
    pub any_attributes: Option<Field>,
}

impl StructFields {
    pub fn add(&mut self, field: &syn::Field) {
        if let Type::Path(ty) = &field.ty {
            let type_path = &ty.path;
            let name = field.ident.as_ref().unwrap();
            let segment = type_path.segments.first().expect("Empty segments in Field");

            let type_path = if &segment.ident == "Option" {
                unpack_generic_argument(&segment.arguments)
            } else if &segment.ident == "Vec" {
                unpack_generic_argument(&segment.arguments)
            } else if &segment.ident == "Box" {
                unpack_generic_argument(&segment.arguments)
            } else {
                type_path
            };
            self.push(type_path, &segment.ident, name);
        }
    }

    fn push(&mut self, ty: &Path, generic_type: &Ident, name: &Ident) {
        if &ty.segments.len() > &1 {
            // Type with scope (elements::, attributes::, groups::)
            let type_scope = &ty.segments[0].ident;
            let type_name = &ty.segments[1].ident;
            let field = Field::new(name, type_name, Some(type_scope), generic_type);

            if type_scope == "attributes" {
                if type_name == "RawAttribute" {
                    self.any_attributes = Some(field);
                } else {
                    self.attributes.push(field);
                }
            } else if type_scope == "elements" {
                if type_name == "RawElement" {
                    self.any_elements = Some(field);
                } else {
                    self.elements.push(field);
                }
            } else if type_scope == "groups" {
                self.groups.push(field)
            }
        } else {
            let field = Field::new(name, &ty.segments[0].ident, None, generic_type);
            self.texts.push(field)
        }
    }

    pub fn define_fields(&self) -> TokenStream {
        let map_fields = |collection: &[_]| {
            collection
                .iter()
                .map(Field::define_line)
                .fold(quote!(), |mut acc, x| {
                    acc.extend(x);
                    acc
                })
        };
        let mut result = map_fields(&self.elements);
        result.extend(map_fields(&self.groups));
        result.extend(map_fields(&self.attributes));

        result.extend(self.texts.iter().map(Field::define_text_line).fold(
            quote!(),
            |mut acc, x| {
                acc.extend(x);
                acc
            },
        ));

        if let Some(any_elements) = &self.any_elements {
            result.extend(any_elements.define_line())
        }

        if let Some(any_attributes) = &self.any_attributes {
            result.extend(any_attributes.define_line())
        }

        result
    }

    pub fn match_elements(&self) -> TokenStream {
        let mut result =
            self.elements
                .iter()
                .map(Field::element_match_line)
                .fold(quote!(), |mut acc, x| {
                    acc.extend(x);
                    acc
                });

        result.extend(self.groups.iter().map(Field::group_match_line).fold(
            quote!(),
            |mut acc, x| {
                acc.extend(x);
                acc
            },
        ));

        result.extend(default_element_case(&self.any_elements));

        quote! (
            for ch in node.children().filter(|n| n.is_element()) {
                match ch.tag_name().name() {
                    #result
                }
            }
        )
    }

    pub fn match_attributes(&self) -> TokenStream {
        let mut result = self
            .attributes
            .iter()
            .map(Field::attribute_match_line)
            .fold(quote!(), |mut acc, x| {
                acc.extend(x);
                acc
            });

        result.extend(default_attribute_case(&self.any_attributes));

        quote! (
            for attr in node.attributes() {
                match attr.name() {
                    #result
                }
            }
        )
    }

    pub fn assign_lines(&self) -> TokenStream {
        let map_fields = |collection: &[_]| {
            collection
                .iter()
                .map(Field::assigment_line)
                .fold(quote!(), |mut acc, x| {
                    acc.extend(x);
                    acc
                })
        };
        let mut result = map_fields(&self.elements);
        result.extend(map_fields(&self.groups));
        result.extend(map_fields(&self.attributes));
        result.extend(map_fields(&self.texts));

        if let Some(any_elements) = &self.any_elements {
            result.extend(any_elements.assigment_line())
        }

        if let Some(any_attributes) = &self.any_attributes {
            result.extend(any_attributes.assigment_line())
        }

        result
    }
}

fn default_element_case(field: &Option<Field>) -> TokenStream {
    if let Some(any_elements) = field {
        let name = &any_elements.name;
        let ty = any_elements.full_type();
        quote! (_ => {#name.push(#ty::parse(ch)?)})
    } else {
        quote! (_ => {Err(format!("Unexpected element: {:#?}", ch))?})
    }
}

fn default_attribute_case(field: &Option<Field>) -> TokenStream {
    if let Some(any_attr) = field {
        let name = &any_attr.name;
        let ty = any_attr.full_type();
        quote! (_ => {#name.push(#ty::parse(attr)?)})
    } else {
        quote! (_ => {Err(format!("Unexpected attribute: {:#?}", attr))?})
    }
}
