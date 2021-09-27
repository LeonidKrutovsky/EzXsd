pub mod all;
pub mod annotation;
pub mod any;
pub mod any_attribute;
pub mod app_info;
pub mod attribute;
pub mod attribute_group;
pub mod choice;
pub mod complex_content;
pub mod complex_type;
pub mod documentation;
pub mod element;
pub mod enumeration;
pub mod extension;
pub mod field;
pub mod fraction_digits;
pub mod group;
pub mod import;
pub mod include;
pub mod key;
pub mod key_ref;
pub mod length;
pub mod list;
pub mod max_exclusive;
pub mod max_inclusive;
pub mod max_length;
pub mod min_exclusive;
pub mod min_inclusive;
pub mod min_length;
pub mod notation;
pub mod pattern;
pub mod redefine;
pub mod restriction;
pub mod schema;
pub mod selector;
pub mod sequence;
pub mod simple_content;
pub mod simple_type;
pub mod total_digits;
pub mod union;
pub mod unique;
pub mod white_space;

use crate::model::simple_types::QName;
use roxmltree::Node;
use std::convert::{TryFrom, TryInto};

pub use all::All;
pub use annotation::Annotation;
pub use any_attribute::AnyAttribute;
pub use app_info::AppInfo;
pub use attribute::LocalAttribute;
pub use attribute::TopLevelAttribute;
pub use attribute_group::AttributeGroup;
pub use attribute_group::AttributeGroupRef;
pub use choice::Choice;
pub use choice::SimpleChoice;
pub use complex_content::ComplexContent;
pub use documentation::Documentation;
pub use element::Element;
pub use enumeration::Enumeration;
pub use extension::Extension;
pub use extension::SimpleExtension;
pub use field::Field;
pub use fraction_digits::FractionDigits;
pub use import::Import;
pub use include::Include;
pub use length::Length;
pub use max_exclusive::MaxExclusive;
pub use max_inclusive::MaxInclusive;
pub use max_length::MaxLength;
pub use min_exclusive::MinExclusive;
pub use min_inclusive::MinInclusive;
pub use min_length::MinLength;
pub use pattern::Pattern;
pub use redefine::Redefine;
pub use restriction::ComplexRestriction;
pub use restriction::SimpleRestriction;
pub use selector::Selector;
pub use sequence::Sequence;
pub use sequence::SimpleSequence;
pub use simple_content::SimpleContent;
pub use simple_type::LocalSimpleType;
pub use simple_type::TopLevelSimpleType;
pub use total_digits::TotalDigits;
pub use white_space::WhiteSpace;

use crate::model::attributes;

#[derive(Debug, Default)]
pub struct RawElement {
    name: QName,
    attributes: Vec<attributes::RawAttribute>,
    text: Option<String>,
}

impl RawElement {
    fn parse(value: Node<'_, '_>) -> Result<Self, String> {
        let name = value.tag_name().name().parse()?;
        let prefix = if let Some(p) = value.tag_name().namespace() {
            Some(p.parse()?)
        } else {
            None
        };
        Ok(Self {
            name: QName { prefix, name },
            attributes: value
                .attributes()
                .iter()
                .map(|a| a.try_into())
                .collect::<Result<Vec<_>, _>>()?,

            text: value.text().map(String::from),
        })
    }
}

impl TryFrom<roxmltree::Node<'_, '_>> for RawElement {
    type Error = String;

    fn try_from(value: Node<'_, '_>) -> Result<Self, Self::Error> {
        let name = value.tag_name().name().parse()?;
        let prefix = if let Some(p) = value.tag_name().namespace() {
            Some(p.parse()?)
        } else {
            None
        };
        Ok(Self {
            name: QName { prefix, name },
            attributes: value
                .attributes()
                .iter()
                .map(|a| a.try_into())
                .collect::<Result<Vec<_>, _>>()?,

            text: value.text().map(String::from),
        })
    }
}

#[derive(Debug, Default)]
pub struct AnyElements(pub Vec<RawElement>);

impl AnyElements {
    pub fn push(&mut self, elem: RawElement) {
        self.0.push(elem);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl TryFrom<roxmltree::Children<'_, '_>> for AnyElements {
    type Error = String;

    fn try_from(value: roxmltree::Children<'_, '_>) -> Result<Self, Self::Error> {
        Ok(AnyElements(
            value.map(|n| n.try_into()).collect::<Result<Vec<_>, _>>()?,
        ))
    }
}
