use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use crate::utils::unpack_generic_argument;

#[derive(Debug)]
pub struct DefaultArgument {
    pub name: syn::Ident,
    pub eq_token: syn::Token![=],
    pub value: Option<syn::LitBool>,
}

impl Parse for DefaultArgument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub trait FieldWrapper {
    fn name(&self) -> &Ident;
    fn full_type(&self) -> &syn::Path;
    fn default_value(&self) -> Option<syn::LitBool>;
    fn is_sequence_group(&self) -> bool;
    fn field_type(&self) -> (Option<GenericType>, FieldType);
    fn define_line(&self) -> proc_macro2::TokenStream;
}

impl FieldWrapper for syn::Field {
    //annotation or text
    fn name(&self) -> &Ident {
        self.ident.as_ref().expect("Only named fields support")
    }

    //e.g. Option<elements::Annotation> or groups::ElementModel
    fn full_type(&self) -> &syn::Path {
        if let syn::Type::Path(type_path) = &self.ty {
            &type_path.path
        } else {
            unreachable!()
        }
    }

    fn default_value(&self) -> Option<syn::LitBool> {
        if self.attrs.is_empty() {
            None
        } else {
            let attr = &self.attrs[0];
            let res: syn::Result<syn::LitBool> = attr.parse_args();
            res.ok()
        }
    }

    fn is_sequence_group(&self) -> bool {
        if self.attrs.is_empty() {
            false
        } else {
            &self.attrs[0].path.segments[0].ident == "sequence_group"
        }
    }


    fn field_type(&self) -> (Option<GenericType>, FieldType) {
        let generic_type = GenericType::new(&self.full_type().segments[0].ident);
        let ident = &if generic_type.is_some() {
            unpack_generic_argument(&self.full_type().segments[0].arguments)
        } else {
            &self.full_type()
        }.segments[0].ident;

        let field_type = if ident == "elements" {
            FieldType::Element
        } else if ident == "attributes" {
            FieldType::Attribute
        } else if ident == "groups" {
            if self.is_sequence_group() {
                FieldType::SequenceGroup
            } else {
                FieldType::ChoiceGroup
            }
        } else if ident == "String" {
            FieldType::Text
        } else {
            unreachable!("Unknown Field type")
        };
        (generic_type, field_type)
    }

    fn define_line(&self) -> TokenStream {
        let name = self.name();
        let ty = self.full_type();
        let (generic_type, field_type) = self.field_type();


        if let Some(gt) = generic_type {
            match gt {
                GenericType::Option => {
                    if field_type == FieldType::Text {
                        quote! (let #name: #ty = node.text().map(|s| s.to_string());)
                    } else {
                        quote! (let mut #name: #ty = None;)
                    }

                }
                GenericType::Vec => quote! (let mut #name: #ty = vec![];),
                GenericType::Box => quote! (let mut #name: Option<#ty> = None;),
            }
        } else {
            if self.is_sequence_group() {
                quote! (let #name = #ty::parse(node)?;)
            } else if field_type == FieldType::Text {
                quote! (let #name: #ty = node.text().map(|s| s.to_string())?;)
            } else {
                quote! (let mut #name: Option<#ty> = None;)
            }
        }
    }
}

pub trait NamedFields {
    fn impl_parse(&self) -> proc_macro2::TokenStream;
}

impl NamedFields for syn::FieldsNamed {
    fn impl_parse(&self) -> TokenStream {
        let mut fields_define = quote!();
        let mut fields_match = quote!();
        let mut attributes_match = quote!();
        let mut assign_lines = quote!();

        for field in &self.named {
            fields_define.extend(field.define_line());
        }

        quote!(
            pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
                #fields_define
                #fields_match
                #attributes_match
                Ok(Self{
                    #assign_lines
                })
            }
        )
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: Ident,
    pub type_name: Ident,
    pub type_scope: Option<Ident>,
    pub generic_type: Option<GenericType>,
}

impl Field {
    pub const COMPLEX_GROUPS: &'static [&'static str] =
        &["AttrDecls", "ComplexTypeModel", "ElementModel", "SimpleRestrictionModel"];

    pub fn new(
        name: &Ident,
        type_name: &Ident,
        type_scope: Option<&Ident>,
        generic_type: &Ident,
    ) -> Self {
        let generic_type = if generic_type == "Option" {
            Some(GenericType::Option)
        } else if generic_type == "Vec" {
            Some(GenericType::Vec)
        } else if generic_type == "Box" {
            Some(GenericType::Box)
        } else {
            None
        };

        Self {
            name: name.clone(),
            type_name: type_name.clone(),
            type_scope: type_scope.map(|t| t.clone()),
            generic_type,
        }
    }

    pub fn full_type(&self) -> TokenStream {
        let type_name = &self.type_name;
        if let Some(type_scope) = &self.type_scope {
            quote!(#type_scope::#type_name)
        } else {
            quote!(#type_name)
        }
    }

    pub fn define_line(&self) -> TokenStream {
        let name = &self.name;
        let ty = &self.full_type();

        if Self::COMPLEX_GROUPS.contains(&self.type_name.to_string().as_ref()) {
            return quote! (
                let #name = #ty::parse(node)?;
            );
        }

        if let Some(gt) = &self.generic_type {
            match gt {
                GenericType::Option => quote! (
                    let mut #name: Option<#ty> = None;
                ),
                GenericType::Vec => quote! (
                    let mut #name: Vec<#ty> = vec![];
                ),
                GenericType::Box => quote! (
                    let mut #name: Option<Box<#ty>> = None;
                ),
            }
        } else {
            quote! (let mut #name: Option<#ty> = None;)
        }
    }

    pub fn define_text_line(&self) -> TokenStream {
        let name = &self.name;
        let ty = self.full_type();
        if let Some(gt) = &self.generic_type {
            match gt {
                GenericType::Option => quote! (
                    let #name: Option<#ty> = node.text().map(|s| s.to_string());
                ),
                GenericType::Vec => unreachable!("Vec of texts are not supported"),
                GenericType::Box => unreachable!("Box of text are not supported"),
            }
        } else {
            quote! (let #name: #ty = node.text().map(|s| s.to_string());)
        }
    }

    pub fn group_match_line(&self) -> TokenStream {
        if Self::COMPLEX_GROUPS.contains(&self.type_name.to_string().as_ref()) {
            return quote!();
        }
        let ty = self.full_type();
        self.match_line(quote! (some_tag_name if #ty::NAMES.contains(&some_tag_name)))
    }

    pub fn element_match_line(&self) -> TokenStream {
        let ty = self.full_type();
        self.match_line(quote! (#ty::NAME))
    }

    pub fn attribute_match_line(&self) -> TokenStream {
        let name = &self.name;
        let ty = self.full_type();
        let parse_line = quote! (
            #ty::parse(attr)?
        );
        if let Some(generic_type) = &self.generic_type {
            match generic_type {
                GenericType::Option => quote! (
                    #ty::NAME => {#name = Some(#parse_line)},
                ),
                GenericType::Vec => quote! (
                    #ty::NAME => {#name.push(#parse_line)},
                ),
                GenericType::Box => quote! (
                    #ty::NAME => {#name = Some(Box::new(#parse_line))},
                ),
            }
        } else {
            quote! (
                #ty::NAME => {#name = Some(#parse_line)},
            )
        }
    }

    pub fn assigment_line(&self) -> TokenStream {
        let name = &self.name;
        if Self::COMPLEX_GROUPS.contains(&self.type_name.to_string().as_ref()) {
            return quote!(#name, );
        }

        let expect_msg = format!("Required field: {}", name);
        if let Some(generic_type) = &self.generic_type {
            match generic_type {
                GenericType::Option => quote! (#name, ),
                GenericType::Vec => quote! (#name, ),
                GenericType::Box => quote! (#name: #name.expect(#expect_msg), ),
            }
        } else {
            quote! (#name: #name.expect(#expect_msg), )
        }
    }

    fn match_line(&self, match_pattern: TokenStream) -> TokenStream {
        let name = &self.name;
        let ty = self.full_type();
        if let Some(generic_type) = &self.generic_type {
            match generic_type {
                GenericType::Option => quote! (
                    #match_pattern => {#name = Some(#ty::parse(ch)?)},
                ),
                GenericType::Vec => quote! (
                    #match_pattern => {#name.push(#ty::parse(ch)?)},
                ),
                GenericType::Box => quote! (
                    #match_pattern => {#name = Some(Box::new(#ty::parse(ch)?))},
                ),
            }
        } else {
            quote! (
                #match_pattern => {#name = Some(#ty::parse(ch)?)},
            )
        }
    }
}

#[derive(Debug)]
pub enum GenericType {
    Option,
    Vec,
    Box,
}

impl GenericType {
    pub fn new(ident: &Ident) -> Option<Self> {
        Some (if ident == "Option" {
            Self::Option
        } else if ident == "Vec" {
            Self::Vec
        } else if ident == "Box" {
            Self::Box
        } else {
            return None;
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum FieldType {
    Element,
    Attribute,
    ChoiceGroup,
    SequenceGroup,
    Text
}

impl FieldType {
    pub fn new(ident: &Ident) -> Option<Self> {
        Some (if ident == "element" {
            Self::Element
        } else if ident == "attribute" {
            Self::Attribute
        } else if ident == "group" {
            Self::ChoiceGroup
        } else {
            return None;
        })
    }
}
