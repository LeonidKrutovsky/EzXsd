use crate::model::elements::all::AllType;
use crate::model::elements::choice::Choice;
use crate::model::elements::group::Group;
use crate::model::elements::sequence::Sequence;

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
#[derive(Debug)]
pub enum TypeDefParticle {
    Group(Group),
    All(AllType),
    Choice(Choice),
    Sequence(Sequence),
}
