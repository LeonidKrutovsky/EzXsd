use crate::model::attributes;
use crate::model::elements;
use xml_utils::element;

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
#[element(name = "keyref")]
pub struct KeyRef {
    pub annotation: Option<elements::Annotation>,
    pub selector: elements::Selector,
    pub fields: Vec<elements::Field>,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
    pub name: attributes::Name,
    pub refer: attributes::Refer,
}
