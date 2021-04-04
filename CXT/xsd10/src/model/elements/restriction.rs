use crate::model::complex_types::complex_restriction_type::ComplexRestrictionType;
use crate::model::complex_types::simple_restriction_type::SimpleRestrictionType;
use crate::model::elements::annotation::Annotation;
use crate::model::groups::simple_restriction_model::SimpleRestrictionModel;
use crate::model::attributes::id::Id;
use crate::model::attributes::base::Base;
use crate::model::attributes::AnyAttributes;

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
#[derive(Default, Debug)]
pub struct Restriction {
    pub annotation: Option<Annotation>,
    pub model: SimpleRestrictionModel,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub base: Option<Base>, // base attribute and simpleType child are mutually exclusive, but one or other is required
}

// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:simpleRestrictionType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:simpleContent
pub type SimpleRestriction = SimpleRestrictionType;

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:complexRestrictionType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:complexContent

pub type ComplexRestriction = ComplexRestrictionType;
