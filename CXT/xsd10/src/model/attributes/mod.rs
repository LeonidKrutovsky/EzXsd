pub mod abstract_;
pub mod attribute_form_default;
pub mod base;
pub mod block;
pub mod block_default;
pub mod default;
pub mod element_form_default;
pub mod final_;
pub mod final_default;
pub mod fixed;
pub mod form;
pub mod id;
pub mod item_type;
pub mod language;
pub mod max_occurs;
pub mod member_types;
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
pub mod value;
pub mod version;
pub mod xpath;

pub use abstract_::Abstract;
pub use attribute_form_default::AttributeFormDefault;
pub use base::Base;
pub use block::Block;
pub use block::DerivationBlock;
pub use block_default::BlockDefault;
pub use default::Default_;
pub use element_form_default::ElementFormDefault;
pub use final_::Final;
pub use final_::SimpleFinal;
pub use final_default::FinalDefault;
pub use fixed::Fixed;
pub use fixed::FixedBool;
pub use form::Form;
pub use id::Id;
pub use item_type::ItemType;
pub use language::Language;
pub use max_occurs::MaxOccurs;
pub use max_occurs::MaxOccursBool;
pub use max_occurs::MaxOccursOne;
pub use member_types::MemberTypes;
pub use min_occurs::MinOccurs;
pub use min_occurs::MinOccursBool;
pub use mixed::Mixed;
pub use name::Name;
pub use namespace::Namespace;
pub use namespace::NamespaceUri;
pub use nillable::Nillable;
pub use process_contents::ProcessContents;
pub use public::Public;
pub use ref_::Ref;
pub use refer::Refer;
pub use schema_location::SchemaLocation;
pub use source::Source;
pub use substitution_group::SubstitutionGroup;
pub use system::System;
pub use target_namespace::TargetNamespace;
pub use type_::Type;
pub use use_::Use;
pub use value::NonNegativeValue;
pub use value::PatternValue;
pub use value::PositiveValue;
pub use value::Value;
pub use value::WhiteSpaceValue;
pub use version::Version;
pub use xpath::FieldXPath;
pub use xpath::XPath;

use crate::model::simple_types::{AnySimpleType, QName};
use std::convert::TryFrom;

#[derive(Debug, Default)]
pub struct RawAttribute {
    pub name: QName,
    pub value: AnySimpleType,
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

    pub fn parse(attr: &roxmltree::Attribute) -> Result<Self, String> {
        let mut ns = None;
        if let Some(namespace) = attr.namespace() {
            ns = Some(namespace.parse()?)
        }
        Ok(Self {
            name: QName {
                prefix: ns,
                name: attr.name().parse()?,
            },
            value: attr.value().to_string(),
        })
    }

    pub fn text(&self) -> String {
        format!(" {}=\"{}\"", self.name, self.value)
    }
}

impl TryFrom<&roxmltree::Attribute<'_>> for RawAttribute {
    type Error = String;

    fn try_from(attr: &roxmltree::Attribute) -> Result<Self, Self::Error> {
        let mut ns = None;
        if let Some(namespace) = attr.namespace() {
            ns = Some(namespace.parse()?)
        }
        Ok(Self {
            name: QName {
                prefix: ns,
                name: attr.name().parse()?,
            },
            value: attr.value().to_string(),
        })
    }
}
