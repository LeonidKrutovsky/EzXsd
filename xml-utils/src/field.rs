use proc_macro2::{Ident, TokenStream};
use quote::quote;

#[derive(Debug)]
pub struct Field {
    pub name: Ident,
    pub type_name: Ident,
    pub type_scope: Option<Ident>,
    pub generic_type: Option<GenericType>,
}

impl Field {
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
        let ty = self.full_type();
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

    pub fn group_match_line(&self) -> TokenStream {
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
                #ty::NAME => {#name = Some(#ty::parse(ch)?)},
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
