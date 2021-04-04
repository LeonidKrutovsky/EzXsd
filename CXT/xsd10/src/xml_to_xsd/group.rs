use crate::model::complex_types::named_group::{ContentChoice, NamedGroup};
use crate::model::complex_types::simple_explicit_group::SimpleExplicitGroup;
use crate::model::elements::ElementType;
use crate::model::Annotation;
use crate::xml_to_xsd::XsdNode;
use roxmltree::Node;
use std::convert::TryInto;
use crate::model::attributes::AnyAttributes;

impl NamedGroup {
    pub fn parse(node: Node<'_, '_>) -> Result<Self, String> {
        let mut annotation = None;
        let mut content_choice = None;
        let mut attributes = AnyAttributes::default();
        let mut id = None;
        let mut name = None;

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
                ty => content_choice = Some(ContentChoice::parse(ch, ty)?),
            }
        }

        let content_choice = content_choice
            .ok_or_else(|| format!("Content required for xsd:namedGroup type: {:?}", node))?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.try_into()?),
                "name" => name = Some(attr.try_into()?),
                _ => attributes.push(attr.try_into()?),
            };
        }

        let name =
            name.ok_or_else(|| format!("Name required for xsd:namedGroup type: {:?}", node))?;

        Ok(Self {
            annotation,
            content_choice,
            attributes,
            id,
            name,
        })
    }
}

impl ContentChoice {
    pub fn parse(node: Node<'_, '_>, xsd_type: ElementType) -> Result<Self, String> {
        Ok(match xsd_type {
            ElementType::All => unimplemented!("Not present in ONVIF"), //Self::All(All::parse(node)?),
            ElementType::Choice => Self::Choice(SimpleExplicitGroup::parse(node)?),
            ElementType::Sequence => Self::Sequence(SimpleExplicitGroup::parse(node)?),
            _ => {
                return Err(format!(
                    "Invalid content type of xsd:namedGroup: {:?}",
                    node
                ))
            }
        })
    }
}
