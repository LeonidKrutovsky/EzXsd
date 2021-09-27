use crate::model::attributes;
use crate::model::elements;
use crate::model::groups;
use xml_utils::element;

// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]        from type xsd:annotated
//      xsd:simpleType [0..1]        from group xsd:simpleRestrictionModel
//      Choice [0..*]                from group xsd:facets
//          xsd:minExclusive
//          xsd:minInclusive
//          xsd:maxExclusive
//          xsd:maxInclusive
//          xsd:totalDigits
//          xsd:fractionDigits
//          xsd:length
//          xsd:minLength
//          xsd:maxLength
//          xsd:enumeration
//          xsd:whiteSpace
//          xsd:pattern
//
// Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
// base	            [0..1]	xsd:QName
//
// Used in
// Group xsd:simpleDerivation
// Type xsd:simpleType via reference to xsd:simpleDerivation
// Type xsd:localSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
// Type xsd:topLevelSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
#[element(name = "restriction")]
pub struct Restriction {
    pub annotation: Option<elements::Annotation>,
    pub model: groups::SimpleRestrictionModel,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub base: Option<attributes::Base>, // base attribute and simpleType child are mutually exclusive, but one or other is required
}

// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:simpleRestrictionType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:simpleContent
#[element(name = "restriction")]
pub struct SimpleRestriction {
    pub annotation: Option<elements::Annotation>,
    pub model: groups::SimpleRestrictionModel,
    pub attr_decls: groups::AttrDecls,
    pub id: Option<attributes::Id>,
    pub base: attributes::Base,
    pub attributes: Vec<attributes::RawAttribute>,
}

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:complexRestrictionType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:complexContent
#[element(name = "restriction")]
pub struct ComplexRestriction {
    pub annotation: Option<elements::Annotation>,
    pub type_def_particle: Option<groups::TypeDefParticle>,
    pub attr_decls: groups::AttrDecls,
    pub attributes: Vec<attributes::RawAttribute>,
    pub id: Option<attributes::Id>,
    pub base: attributes::Base,
}
