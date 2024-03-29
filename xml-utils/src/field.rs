use crate::utils::unpack_generic_argument;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub enum DefaultArg {
    Bool(syn::LitBool),
    Ident(syn::Ident),
    Int(syn::LitInt),
    Str(syn::LitStr),
    Empty,
}

pub trait FieldWrapper {
    fn name(&self) -> &Ident;
    fn full_type(&self) -> &syn::Path;
    fn type_path(&self) -> &syn::Path;
    fn default_value(&self) -> Option<DefaultArg>;
    fn is_sequence_group(&self) -> bool;
    fn field_type(&self) -> (Option<GenericType>, FieldType);
    fn define_line(&self) -> proc_macro2::TokenStream;
    fn match_element_line(&self) -> Option<proc_macro2::TokenStream>;
    fn assigment_line(&self, elem_name: &syn::LitStr) -> proc_macro2::TokenStream;
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

    fn type_path(&self) -> &syn::Path {
        let segment = &self.full_type().segments[0];
        let generic_type = GenericType::new(&segment.ident);
        if generic_type.is_some() {
            unpack_generic_argument(&segment.arguments)
        } else {
            &self.full_type()
        }
    }

    fn default_value(&self) -> Option<DefaultArg> {
        if self.attrs.is_empty() {
            return None;
        }
        let attr = &self.attrs[0];
        if attr.path.segments[0].ident != "default" {
            return None;
        }

        let res = if let Ok(lit_bool) = attr.parse_args::<syn::LitBool>() {
            DefaultArg::Bool(lit_bool)
        } else if let Ok(lit_int) = attr.parse_args::<syn::LitInt>() {
            DefaultArg::Int(lit_int)
        } else if let Ok(ident) = attr.parse_args::<syn::Ident>() {
            DefaultArg::Ident(ident)
        } else if let Ok(ident) = attr.parse_args::<syn::LitStr>() {
            DefaultArg::Str(ident)
        } else {
            DefaultArg::Empty
        };
        Some(res)
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
        let segments = &if generic_type.is_some() {
            unpack_generic_argument(&self.full_type().segments[0].arguments)
        } else {
            &self.full_type()
        }
        .segments;

        let ident = &segments[0].ident;
        let ty = &segments.last().unwrap().ident;

        let field_type = if ident == "elements" {
            if ty == "RawElement" {
                FieldType::RawElement
            } else {
                FieldType::Element
            }
        } else if ident == "attributes" {
            if ty == "RawAttribute" {
                FieldType::RawAttribute
            } else {
                FieldType::Attribute
            }
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
                quote! (let mut #name = #ty::default();)
            } else if field_type == FieldType::Text {
                quote! (let #name: #ty = node.text().map(|s| s.to_string())?;)
            } else {
                quote! (let mut #name: Option<#ty> = None;)
            }
        }
    }

    fn match_element_line(&self) -> Option<proc_macro2::TokenStream> {
        let name = self.name();
        let ty = self.type_path();
        let (generic_type, field_type) = self.field_type();

        let match_line = match field_type {
            FieldType::Element | FieldType::Attribute => quote!(#ty::NAME),
            FieldType::SequenceGroup | FieldType::ChoiceGroup => {
                quote!(__tn__ if #ty::NAMES.contains(&__tn__))
            }
            FieldType::RawElement | FieldType::RawAttribute => quote!(&_),
            FieldType::Text => return None,
        };

        let parse = quote! (#ty::parse(__value__)?);
        let result = if let Some(gt) = generic_type {
            match gt {
                GenericType::Option => quote!(#match_line => #name = Some(#parse),),
                GenericType::Vec => quote!(#match_line => #name.push(#parse),),
                GenericType::Box => quote!(#match_line => #name = Some(Box::new(#parse)),),
            }
        } else {
            if field_type == FieldType::SequenceGroup {
                quote!(#match_line => #name.push(__value__)?,)
            } else {
                quote!(#match_line => #name = Some(#parse),)
            }
        };

        Some(result)
    }

    fn assigment_line(&self, elem_name: &syn::LitStr) -> proc_macro2::TokenStream {
        let name = self.name();
        let expect_msg = format!(
            "Invalid <xsd:{}> element. Required field: {}",
            elem_name.value(),
            name
        );
        let (generic_type, field_type) = self.field_type();
        if let Some(gt) = generic_type {
            match gt {
                GenericType::Option | GenericType::Vec => quote! (#name, ),
                GenericType::Box => quote! (#name: #name.expect(#expect_msg), ),
            }
        } else {
            if field_type == FieldType::SequenceGroup {
                quote! (#name,)
            } else if let Some(dv) = self.default_value() {
                let ty = self.type_path();
                let default_val = match dv {
                    DefaultArg::Bool(v) => quote! (#ty{0: #v.into()}),
                    DefaultArg::Ident(v) => quote! (#ty{0: #v.into()}),
                    DefaultArg::Int(v) => quote! (#ty{0: #v.into()}),
                    DefaultArg::Str(v) => quote! (#ty{0: #v.into()}),
                    DefaultArg::Empty => quote! (#ty::default()),
                };
                quote! (#name: #name.unwrap_or(#default_val),)
            } else {
                quote! (#name: #name.ok_or_else(|| format!("{}. Node: {:?}", #expect_msg, node))?, )
            }
        }
    }
}

pub trait NamedFields {
    fn impl_parse(&self, elem_name: &syn::LitStr) -> proc_macro2::TokenStream;
    fn impl_text(&self, elem_name: &syn::LitStr) -> proc_macro2::TokenStream;
}

impl NamedFields for syn::FieldsNamed {
    fn impl_parse(&self, elem_name: &syn::LitStr) -> proc_macro2::TokenStream {
        let mut fields_define = quote!();
        let mut elements_match = quote!();
        let mut attributes_match = quote!();
        let mut assign_lines = quote!();

        let mut default_element: Option<proc_macro2::TokenStream> = None;
        let mut default_attribute: Option<proc_macro2::TokenStream> = None;

        for field in &self.named {
            fields_define.extend(field.define_line());
            let (_generic_type, field_type) = field.field_type();
            if let Some(match_line) = field.match_element_line() {
                match field_type {
                    FieldType::Element | FieldType::SequenceGroup | FieldType::ChoiceGroup => {
                        elements_match.extend(match_line)
                    }
                    FieldType::RawElement => default_element = Some(match_line),
                    FieldType::Attribute => attributes_match.extend(match_line),
                    FieldType::RawAttribute => default_attribute = Some(match_line),
                    FieldType::Text => {}
                }
            }
            assign_lines.extend(field.assigment_line(elem_name))
        }

        if let Some(de) = default_element {
            elements_match.extend(de);
        } else {
            elements_match.extend(quote!(&_ => Err(format!("Invalid <xsd:{}> element. Unexpected node: {:?}", #elem_name, __value__))?));
        }

        if let Some(da) = default_attribute {
            attributes_match.extend(da);
        } else {
            attributes_match.extend(quote!(&_ => Err(format!("Invalid <xsd:{}> element. Unexpected attribute: {:?}", #elem_name, __value__))?));
        }

        quote!(
            pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
                #fields_define
                for __value__ in node.children().filter(|n| n.is_element()) {
                    match __value__.tag_name().name() {
                        #elements_match
                    }
                }
                for __value__ in node.attributes() {
                    match __value__.name() {
                        #attributes_match
                    }
                }

                Ok(Self{
                    #assign_lines
                })
            }
        )
    }
    fn impl_text(&self, elem_name: &syn::LitStr) -> proc_macro2::TokenStream {
        let mut attributes_text = quote!();

        quote! {
            pub fn text(&self) -> String {
                format!("<{0}> </{0}>", Self::NAME)
            }
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
        Some(if ident == "Option" {
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
    RawElement,
    Attribute,
    RawAttribute,
    ChoiceGroup,
    SequenceGroup,
    Text,
}
