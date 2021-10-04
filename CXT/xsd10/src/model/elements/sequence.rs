use crate::model::{attributes, elements, groups};
use xml_utils::element;

// xsd:sequence
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:simpleExplicitGroup
// Properties: Local, Qualified
//
// Used in
// Type xsd:namedGroup (Element xsd:group)
#[element(name = "sequence")]
pub struct SimpleSequence {
    pub annotation: Option<elements::Annotation>,
    pub nested_particle: Vec<groups::NestedParticle>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
}

// xsd:sequence
// See http://www.w3.org/TR/xmlschema-1/#element-sequence.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Other elements with the same name: xsd:sequence
// Type: xsd:explicitGroup
// Properties: Global, Qualified
//
// Used in
// Group xsd:nestedParticle
// Group xsd:typeDefParticle
// Group xsd:complexTypeModel via reference to xsd:typeDefParticle
// Type xsd:complexType via reference to xsd:complexTypeModel
// Type xsd:complexRestrictionType via reference to xsd:typeDefParticle (Element xsd:restriction)
// Type xsd:extensionType via reference to xsd:typeDefParticle (Element xsd:extension)
// Type xsd:localComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:topLevelComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:explicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
// Type xsd:simpleExplicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
#[element(name = "sequence")]
pub struct Sequence {
    pub annotation: Option<elements::Annotation>,
    pub nested_particle: Vec<groups::NestedParticle>,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    #[default]
    pub min_occurs: attributes::MinOccurs,
    #[default]
    pub max_occurs: attributes::MaxOccurs,
}

#[cfg(test)]
mod test {
    use super::*;
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
        let res:Sequence = Sequence::parse(root).unwrap();
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
