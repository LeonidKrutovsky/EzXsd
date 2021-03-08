use roxmltree::Node;

use crate::model::elements::ElementType;
use crate::xml_to_xsd::XsdNode;

use crate::model::complex_types::explicit_group::ExplicitGroup;
use crate::model::groups::nested_particle::NestedParticle;
use crate::model::Annotation;
use std::convert::TryInto;

impl<'a> ExplicitGroup<'a> {
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
                "id" => res.id = Some(attr.value().parse()?),
                "minOccurs" => res.min_occurs = attr.try_into()?,
                "maxOccurs" => res.max_occurs = attr.try_into()?,
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::model::complex_types::explicit_group::ExplicitGroup;
    use num_bigint::ToBigUint;
    use crate::model::attributes::max_occurs::MaxOccurs::Bounded;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r###"
            <xs:sequence xmlns:xs="http://www.w3.org/2001/XMLSchema" a='a' b='a' maxOccurs='5'>
                <xs:element name="Parameters" type="xs:string" />
                <xs:element name="Parameters2" type="xs:string" />
                <xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>
            </xs:sequence>
            "###,
        )
        .unwrap();
        let root = doc.root_element();
        let res = ExplicitGroup::parse(root).unwrap();
        assert!(res.annotation.is_none());
        let np = &res.nested_particle;
        assert_eq!(np.len(), 3);

        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.min_occurs.0, "1".parse().unwrap());
        match &res.max_occurs {
            Bounded(x) => assert_eq!(x.0, "5".parse().unwrap()),
            _ => unreachable!(),
        }
    }
}
