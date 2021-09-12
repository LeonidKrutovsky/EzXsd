// base
// Attribute information
// Namespace: None
//
// Schema document: datatypes.xsd
//
// Type: xsd:QName
//
// Properties: Local, Unqualified
//
// Value
// A value of type xsd:QName.
// Used in
// Anonymous type of element xsd:restriction
// Type xsd:complexRestrictionType (Element xsd:restriction)
// Type xsd:extensionType (Element xsd:extension)
// Type xsd:simpleExtensionType (Element xsd:extension)
// Type xsd:simpleRestrictionType (Element xsd:restriction)

use crate::model::simple_types::QName;
use xml_utils::*;

#[attribute(name = "base")]
pub struct Base(pub QName);
