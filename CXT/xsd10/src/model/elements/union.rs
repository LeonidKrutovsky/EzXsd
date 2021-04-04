use crate::model::elements::annotation::Annotation;
use crate::model::elements::simple_type::LocalSimpleType;
use crate::model::attributes::id::Id;
use crate::model::attributes::member_types::MemberTypes;
use crate::model::attributes::AnyAttributes;

// xsd:notation
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      xsd:simpleType [0..*]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// memberTypes	    [0..1]	Anonymous
//
// Used in
// Group xsd:schemaTop
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
#[derive(Default, Debug)]
pub struct Union {
    pub annotation: Option<Annotation>,
    pub simple_type: Vec<LocalSimpleType>,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub member_types: MemberTypes,
}
