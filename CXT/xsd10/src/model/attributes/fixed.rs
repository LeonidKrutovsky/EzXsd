use crate::model::simple_types::{String_};

use xml_utils::*;

// fixed
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:string
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd string
//
// Used in
// Type xsd:localAttributeType (Element xsd:attribute)
//  Type xsd:localElement (Element xsd:element)
//  Type xsd:topLevelAttributeType (Element xsd:attribute)
//  Type xsd:topLevelElement (Element xsd:element)
//  Type xsd:narrowMaxMin via derivation of xsd:localElement (Element xsd:element)
#[attribute(name = "fixed")]
pub struct Fixed(pub String_);

// fixed
// Attribute information
// Namespace: None
//
// Schema document: datatypes.xsd
//
// Other attributes with the same name: fixed
//
// Type: xsd:boolean
//
// Properties: Local, Unqualified
//
// Value
// A value of type xsd:boolean.
// Used in
// Anonymous type of element xsd:totalDigits via derivation of xsd:facet
// Anonymous type of element xsd:whiteSpace via derivation of xsd:facet
// Type xsd:facet (Elements xsd:minExclusive, xsd:minInclusive, xsd:maxExclusive, xsd:maxInclusive)
// Type xsd:numFacet via derivation of xsd:facet (Elements xsd:fractionDigits, xsd:length, xsd:minLength, xsd:maxLength)

#[attribute(name = "fixed")]
pub struct FixedBool(pub bool);
