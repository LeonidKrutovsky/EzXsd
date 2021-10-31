use crate::model::simple_types::qname::QName;

use xml_utils::*;

// itemType
// Attribute information
// Namespace: None
// Schema document: datatypes.xsd
// Type: xsd:QName
// Properties: Local, Unqualified
//
// Value
//  A value of type xsd:QName.
// Used in
// Anonymous type of element xsd:list
#[attribute(name = "itemType")]
pub struct ItemType(pub QName);
