use crate::model::elements;
use crate::model::attributes;
use xml_utils::element;

// xsd:simpleContent
// See http://www.w3.org/TR/xmlschema-1/#element-simpleContent.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      Choice [1..1]
//          xsd:restriction
//          xsd:extension
//
// Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
//
// Used in
// Group xsd:complexTypeModel
// Type xsd:complexType via reference to xsd:complexTypeModel
// Type xsd:localComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:topLevelComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
#[element(name = "simpleContent")]
pub struct SimpleContent {
    pub annotation: Option<elements::Annotation>,
    pub content: SimpleContentChoice,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
}

#[derive(Debug)]
pub enum SimpleContentChoice {
    Restriction(Box<elements::SimpleRestriction>),
    Extension(Box<elements::SimpleExtension>),
}

impl Default for SimpleContentChoice {
    fn default() -> Self {
        unimplemented!()
    }
}