use xml_utils::attribute;

// nillable
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:boolean
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd boolean
//
// Used in
// Type xsd:localElement (Element xsd:element)
//  Type xsd:topLevelElement (Element xsd:element)
//  Type xsd:narrowMaxMin via derivation of xsd:localElement (Element xsd:element)
#[attribute(name = "nillable")]
pub struct Nillable(pub bool);
