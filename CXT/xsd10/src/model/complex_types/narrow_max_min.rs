use crate::model::groups;
use crate::model::elements;
use crate::model::attributes;
use xml_utils::complex_type;

// xsd:narrowMaxMin
// restricted max/min
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]
//      Choice [0..1]    from group xsd:elementModel
//          xsd:simpleType
//          xsd:complexType
//      Choice [0..*]    from group xsd:identityConstraint
//          xsd:unique
//          xsd:key
//          xsd:keyref
//
// Attributes
// id	            [0..1]	xsd:ID		                                                from type xsd:annotated
// name	            [0..1]	xsd:NCName		                                            from type xsd:localElement
// ref	            [0..1]	xsd:QName		                                            from type xsd:localElement
// type	            [0..1]	xsd:QName		                                            from type xsd:localElement
// default	        [0..1]	xsd:string		                                            from type xsd:localElement
// fixed	        [0..1]	xsd:string		                                            from type xsd:localElement
// nillable	        [0..1]	xsd:boolean		Default value is "false".                   from type xsd:localElement
// block	        [0..1]	xsd:blockSet		                                        from type xsd:localElement
// form	            [0..1]	xsd:formChoice		                                        from type xsd:localElement
// minOccurs	    [0..1]	Anonymous		Default value is "1".
// maxOccurs	    [0..1]	Anonymous		Default value is "1".
// Any attribute	[0..*]		            Namespace: ##other, Process Contents: lax
//
// Used by
// Element xsd:element
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:localElement
//                  xsd:narrowMaxMin
#[complex_type()]
pub struct NarrowMaxMin {
    pub annotation: Option<elements::Annotation>,
    pub model: groups::ElementModel,
    pub id: Option<attributes::Id>,
    pub name: Option<attributes::Name>,
    pub ref_: Option<attributes::Ref>,
    pub type_: Option<attributes::Type>,
    pub default: Option<attributes::Default_>,
    pub fixed: Option<attributes::Fixed>,
    pub nillable: attributes::Nillable,
    pub block: Option<attributes::Block>,
    pub form: Option<attributes::Form>,
    pub min_occurs: attributes::MinOccursBool,
    pub max_occurs: attributes::MaxOccursBool,
    pub attributes: attributes::AnyAttributes,
}
