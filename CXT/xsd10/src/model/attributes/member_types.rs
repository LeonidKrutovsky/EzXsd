use crate::model::simple_types::qname::QName;
use crate::model::simple_types::xsd_list::XsdList;
use xml_utils::*;

// memberTypes
// Attribute information
// Namespace: None
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Local, Unqualified
//
// Value
// List of:
//  xsd:QName
// Used in
// Anonymous type of element xsd:union
#[attribute(name = "memberTypes")]
pub struct MemberTypes(pub XsdList<QName>);
