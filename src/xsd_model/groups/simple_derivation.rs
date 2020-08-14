use crate::xsd_model::elements::list::List;
use crate::xsd_model::elements::restriction::Restriction;
use crate::xsd_model::elements::union::Union;

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
#[derive(Debug)]
pub enum SimpleDerivation<'a> {
    Restriction(Box<Restriction<'a>>),
    List(Box<List<'a>>),
    Union(Box<Union<'a>>),
}
