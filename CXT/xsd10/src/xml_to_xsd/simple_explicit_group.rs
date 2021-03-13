use crate::model::complex_types::simple_explicit_group::SimpleExplicitGroup;
use crate::model::elements::ElementType;
use crate::model::groups::nested_particle::NestedParticle;
use crate::model::Annotation;
use crate::xml_to_xsd::XsdNode;
use roxmltree::Node;
use std::convert::TryInto;

impl<'a> SimpleExplicitGroup<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => res.annotation = Some(Annotation::parse(ch)?),
                _ => res.nested_particle.push(NestedParticle::parse(ch)?),
            }
        }

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.try_into()?),
                _ => res.attributes.push(attr.try_into()?),
            };
        }

        Ok(res)
    }
}
