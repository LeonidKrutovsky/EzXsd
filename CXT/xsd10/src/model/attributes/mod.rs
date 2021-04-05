pub mod abstract_;
pub mod attribute_form_default;
pub mod block;
pub mod block_default;
pub mod default;
pub mod element_form_default;
pub mod final_;
pub mod final_default;
pub mod fixed;
pub mod form;
pub mod id;
pub mod max_occurs;
pub mod min_occurs;
pub mod mixed;
pub mod name;
pub mod namespace;
pub mod nillable;
pub mod process_contents;
pub mod public;
pub mod ref_;
pub mod refer;
pub mod schema_location;
pub mod source;
pub mod substitution_group;
pub mod system;
pub mod target_namespace;
pub mod type_;
pub mod use_;
pub mod version;
pub mod xpath;
pub mod base;
pub mod value;
pub mod item_type;
pub mod member_types;
pub mod language;

pub use abstract_::Abstract;
pub use attribute_form_default::AttributeFormDefault;
pub use source::Source;
pub use language::Language;
pub use id::Id;
pub use namespace::Namespace;
pub use namespace::NamespaceUri;
pub use process_contents::ProcessContents;
pub use min_occurs::MinOccurs;
pub use max_occurs::MaxOccurs;
pub use mixed::Mixed;
pub use xpath::FieldXPath;
pub use schema_location::SchemaLocation;
pub use name::Name;
pub use refer::Refer;
pub use item_type::ItemType;
pub use public::Public;
pub use system::System;
pub use value::PatternValue;


use crate::model::simple_types::{QName, AnySimpleType};
use std::convert::{TryFrom};

#[derive(Debug, Default)]
pub struct RawAttribute {
    name: QName,
    value: AnySimpleType
}

impl RawAttribute {
    pub fn name(&self) -> &str {
        self.name.name()
    }

    pub fn namespace(&self) -> Option<&str> {
        self.name.prefix()
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

impl TryFrom<&roxmltree::Attribute<'_>> for RawAttribute {
    type Error = String;

    fn try_from(attr: &roxmltree::Attribute) -> Result<Self, Self::Error> {
        let mut ns = None;
        if let Some(namespace) = attr.namespace() {
            ns = Some(namespace.parse()?)
        }
        Ok(Self{
            name: QName{
                prefix: ns,
                name: attr.name().parse()?
            },
            value: attr.value().to_string()
        })
    }
}

#[derive(Debug, Default)]
pub struct AnyAttributes(pub Vec<RawAttribute>);

impl AnyAttributes {
    pub fn push(&mut self, attr: RawAttribute) {
        self.0.push(attr);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

