use crate::model::elements;
use crate::model::attributes;
use xml_utils::complex_type;

// xsd:namedGroupRef
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
// xsd:annotation [0..1]    from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax	            from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                                    from type xsd:annotated
// ref	            [1..1]	xsd:QName
// minOccurs	    [0..1]	xsd:nonNegativeInteger	minimum number of occurrences	Default value is "1".   from group xsd:occurs
// maxOccurs	    [0..1]	Anonymous	            maximum number of occurrences	Default value is "1".   from group xsd:occurs
//
// Used by
// Element xsd:group
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:namedGroupRef
#[complex_type()]
pub struct NamedGroupRef {
    pub annotation: Option<elements::Annotation>,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
    pub ref_: attributes::Ref,
    pub min_occurs: attributes::MinOccurs,
    pub max_occurs: attributes::MaxOccurs,
}
