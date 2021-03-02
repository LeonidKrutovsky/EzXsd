use crate::model::elements::annotation::Annotation;
use crate::model::simple_types::non_negative_integer::NonNegativeInteger;
use crate::model::simple_types::qname::QName;
use crate::model::simple_types::Id;
use crate::model::{MaxOccurs, RawAttribute};

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
#[derive(Debug)]
pub struct NamedGroupRef<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id,
    pub ref_: QName<'a>,
    pub min_occurs: NonNegativeInteger,
    pub max_occurs: MaxOccurs,
}
