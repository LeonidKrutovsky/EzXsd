use crate::model::elements::all::AllType;
use crate::model::elements::choice::Choice;
use crate::model::elements::group::Group;
use crate::model::elements::sequence::Sequence;
use xml_utils::group;

// xsd:typeDefParticle
// 'complexType' uses this
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      xsd:group
//      xsd:all    An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//      xsd:choice
//      xsd:sequence
//
// Used in
// Group xsd:complexTypeModel
// Type xsd:complexRestrictionType (Element xsd:restriction)
// Type xsd:extensionType (Element xsd:extension)
#[group()]
pub enum TypeDefParticle {
    Group(Group),
    All(AllType),
    Choice(Choice),
    Sequence(Sequence),
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_names() {
        assert_eq!(
            TypeDefParticle::NAMES,
            [Group::NAME, AllType::NAME, Choice::NAME, Sequence::NAME]
        );
    }

    #[test]
    fn test_parse_sequence() {
        let xsd = r###"
                <sequence some_attr="value">
                    <element name="Item" type="string" maxOccurs="unbounded" />
                </sequence>
        "###;

        let doc = roxmltree::Document::parse(xsd).unwrap();
        let root = doc.root_element();
        if let TypeDefParticle::Sequence(seq) = TypeDefParticle::parse(root).unwrap() {
            assert!(seq.annotation.is_none());
            assert_eq!(seq.attributes[0].value(), "value");
        } else {
            panic!("Test test_parse_sequence failed")
        }
    }
}
