use crate::model::elements::list::List;
use crate::model::elements::restriction::Restriction;
use crate::model::elements::union::Union;
use xml_utils::group;

// xsd:simpleDerivation
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: datatypes.xsd
//
// Content
//  Choice [1..1]
//      xsd:restriction
//      xsd:list
//      xsd:union
//
// Used in
// Type xsd:simpleType
// Type xsd:localSimpleType (Element xsd:simpleType)
// Type xsd:topLevelSimpleType (Element xsd:simpleType)
#[group()]
pub enum SimpleDerivation {
    Restriction(Box<Restriction>),
    List(Box<List>),
    Union(Box<Union>),
}
