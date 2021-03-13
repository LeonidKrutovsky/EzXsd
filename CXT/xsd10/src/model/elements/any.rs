use crate::model::elements::annotation::Annotation;
use crate::model::attributes::max_occurs::MaxOccurs;
use crate::model::attributes::min_occurs::MinOccurs;
use crate::model::attributes::process_contents::ProcessContents;
use crate::model::attributes::namespace::Namespace;
use crate::model::attributes::id::Id;
use crate::model::attributes::AnyAttributes;

// xsd:any
// See http://www.w3.org/TR/xmlschema-1/#element-any.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
// xsd:annotation [0..1]    from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax	            from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                                    from type xsd:annotated
// namespace	    [0..1]	xsd:namespaceList	    Default value is "##any".                               from type xsd:wildcard
// processContents	[0..1]	Anonymous		        Default value is "strict".                              from type xsd:wildcard
// minOccurs	    [0..1]	xsd:nonNegativeInteger	minimum number of occurrences	Default value is "1".   from group xsd:occurs
// maxOccurs	    [0..1]	Anonymous	            maximum number of occurrences	Default value is "1".   from group xsd:occurs
//
// Used in
// Group xsd:nestedParticle
// Type xsd:explicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
// Type xsd:simpleExplicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
#[derive(Debug, Default)]
pub struct Any<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub namespace: Namespace,
    pub process_contents: ProcessContents,
    pub min_occurs: MinOccurs,
    pub max_occurs: MaxOccurs,
}
