// attributeFormDefault
// Attribute information
// Namespace: None
// Schema document: xmlschema.xsd
// Type: xsd:formChoice
// Properties: Local, Unqualified
//
// Value
// Type based on xsd:NMTOKEN

// Valid value	Description
// qualified	Local declarations are qualified (in a namespace)
// unqualified	Local declarations are unqualified (not in a namespace)

// Used in
// Anonymous type of element xsd:schema

pub enum AttributeFormDefault {
    Qualified,
    Unqualified
}

impl AttributeFormDefault{
    const NAME: &'static str = "attributeFormDefault";
}