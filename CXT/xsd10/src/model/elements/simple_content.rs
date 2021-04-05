use crate::model::elements::annotation::Annotation;
use crate::model::elements::extension::SimpleExtension;
use crate::model::elements::restriction::SimpleRestriction;
use crate::model::attributes::id::Id;
use crate::model::attributes::AnyAttributes;

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
#[derive(Debug, Default)]
pub struct SimpleContent {
    pub annotation: Option<Annotation>,
    pub content: SimpleContentChoice,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
}

#[derive(Debug)]
pub enum SimpleContentChoice {
    Restriction(Box<SimpleRestriction>),
    Extension(Box<SimpleExtension>),
}

impl Default for SimpleContentChoice {
    fn default() -> Self {
        Self::Restriction(Box::new(SimpleRestriction::default()))
    }
}