use crate::model::elements::annotation::Annotation;
use crate::model::elements::simple_type::LocalSimpleType;
use crate::model::simple_types::qname::QName;
use crate::model::simple_types::Id;
use crate::model::RawAttribute;

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      xsd:simpleType [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// itemType	        [0..1]	xsd:QName
//
// Used in
// Group xsd:simpleDerivation
// Type xsd:simpleType via reference to xsd:simpleDerivation
// Type xsd:localSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
// Type xsd:topLevelSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
#[derive(Default, Debug)]
pub struct List<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub simple_type: Option<LocalSimpleType<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id,
    pub item_type: Option<QName>,
}
