use crate::model::elements::annotation::Annotation;
use crate::model::elements::field::Field;
use crate::model::elements::selector::Selector;
use crate::model::attributes::id::Id;
use crate::model::attributes::name::Name;
use crate::model::attributes::refer::Refer;
use crate::model::attributes::AnyAttributes;

// xsd:keyref
// See http://www.w3.org/TR/xmlschema-1/#element-keyref.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]       from type xsd:annotated
//      xsd:selector [1..1]         from type xsd:keybase
//      xsd:field [1..*]
//
// Attributes
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		from type xsd:annotated
// name	            [1..1]	xsd:NCName		from type xsd:keybase
// refer	        [1..1]	xsd:QName
//
// Used in
// Group xsd:identityConstraint
// Group xsd:elementModel via reference to xsd:identityConstraint
// Type xsd:localElement via reference to xsd:elementModel (Element xsd:element)
// Type xsd:narrowMaxMin via reference to xsd:elementModel (Element xsd:element)
// Type xsd:topLevelElement via reference to xsd:elementModel (Element xsd:element)
#[derive(Debug)]
pub struct KeyRef<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub selector: Selector<'a>,
    pub fields: Vec<Field<'a>>,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub name: Name,
    pub refer: Refer,
}
