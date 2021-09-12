use crate::model::attributes;
use crate::model::elements;
use xml_utils::element;

// xsd:union
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: datatypes.xsd
//
// Type: Anonymous
//
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      from type xsd:annotated
//          xsd:annotation [0..1]
//          xsd:simpleType [0..*]
//  Attributes
//      Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
//      id	            [0..1]	xsd:ID		                                            from type xsd:annotated
//      memberTypes	    [0..1]	Anonymous

// Used in
// Group xsd:simpleDerivation
// Type xsd:simpleType via reference to xsd:simpleDerivation
// Type xsd:localSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
// Type xsd:topLevelSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)

// Sample instance
// <xsd:union>
//    <xsd:simpleType>
//       <xsd:restriction>
//          <xsd:simpleType>...
//          </xsd:simpleType>
//          <xsd:minExclusive value="any text content">...
//          </xsd:minExclusive>
//       </xsd:restriction>
//    </xsd:simpleType>
// </xsd:union>

#[element(name = "union")]
pub struct Union {
    pub annotation: Option<elements::Annotation>,
    pub simple_type: Vec<elements::LocalSimpleType>,
    pub attributes: attributes::AnyAttributes,
    pub id: Option<attributes::Id>,
    pub member_types: attributes::MemberTypes,
}
