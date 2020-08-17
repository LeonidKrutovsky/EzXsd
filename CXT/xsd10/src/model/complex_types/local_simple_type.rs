use crate::model::elements::annotation::Annotation;
use crate::model::groups::simple_derivation::SimpleDerivation;
use crate::model::simple_types::Id;
use crate::model::RawAttribute;

// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:localSimpleType
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]
//      Choice [1..1]    from group xsd:simpleDerivation
//          xsd:restriction
//          xsd:list
//          xsd:union
//
// Attributes
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax
//
// Used by
// Element xsd:simpleType
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:simpleType
//                  xsd:localSimpleType
#[derive(Debug)]
pub struct LocalSimpleType<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub content_choice: SimpleDerivation<'a>,
    pub id: Id<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
}
