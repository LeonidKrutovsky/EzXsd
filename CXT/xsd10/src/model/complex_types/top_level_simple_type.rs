use crate::model::groups;
use crate::model::elements;
use crate::model::attributes;
use xml_utils::complex_type;

// xsd:topLevelSimpleType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]
//      Choice [1..1]           from group xsd:simpleDerivation
//          xsd:restriction
//          xsd:list
//          xsd:union
//
// Attributes
// id	            [0..1]	xsd:ID		                                                    from type xsd:annotated
// final	        [0..1]	xsd:simpleDerivationSet		                                    from type xsd:simpleType
// name	            [1..1]	xsd:NCName	            Required at the top level
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax
//
// Used by
// Element xsd:simpleType
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:simpleType
//                  xsd:topLevelSimpleType
#[complex_type()]
pub struct TopLevelSimpleType {
    pub annotation: Option<elements::Annotation>,
    pub content_choice: groups::SimpleDerivation,
    pub id: Option<attributes::Id>,
    pub final_: Option<attributes::SimpleFinal>,
    pub name: attributes::Name,
    pub attributes: attributes::AnyAttributes,
}
