use crate::model::complex_types::all_type;
use crate::model::elements::annotation::Annotation;
use crate::model::elements::element::Element;
use crate::model::attributes::id::Id;
use crate::model::attributes::AnyAttributes;

// xsd:all
// An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements. This declaration is for an "all" group that is a child of xsd:group; its type disallows minOccurs and maxOccurs
// See http://www.w3.org/TR/xmlschema-1/#element-all.
//
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Other elements with the same name: xsd:all
// Type: Anonymous
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      Choice [0..*]                   from group xsd:allModel
//          xsd:element
//
// Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
//
// Used in
// Type xsd:namedGroup (Element xsd:group)
#[derive(Debug)]
pub struct All {
    pub annotation: Option<Annotation>,
    pub elements: Vec<Element>,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
}

// xsd:all
// An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
// See http://www.w3.org/TR/xmlschema-1/#element-all.
//
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Other elements with the same name: xsd:all
// Type: xsd:allType
// Properties: Global, Qualified
//
// Used in
// Group xsd:typeDefParticle
// Group xsd:complexTypeModel via reference to xsd:typeDefParticle
// Type xsd:complexType via reference to xsd:complexTypeModel
// Type xsd:complexRestrictionType via reference to xsd:typeDefParticle (Element xsd:restriction)
// Type xsd:extensionType via reference to xsd:typeDefParticle (Element xsd:extension)
// Type xsd:localComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:topLevelComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
pub type AllType = all_type::AllType;
