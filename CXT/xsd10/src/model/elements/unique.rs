use crate::model::complex_types::key_base::KeyBase;
use xml_utils::element;

// xsd:unique
// See http://www.w3.org/TR/xmlschema-1/#element-unique.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:keybase
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      from type xsd:annotated
//          xsd:annotation [0..1]
//          xsd:selector [1..1]
//          xsd:field [1..*]
//  Attributes
//      Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
//      id	            [0..1]	xsd:ID		                                            from type xsd:annotated
//      name	        [1..1]	xsd:NCName
//
// Used in
// Group xsd:identityConstraint
// Group xsd:elementModel via reference to xsd:identityConstraint
// Type xsd:localElement via reference to xsd:elementModel (Element xsd:element)
// Type xsd:narrowMaxMin via reference to xsd:elementModel (Element xsd:element)
// Type xsd:topLevelElement via reference to xsd:elementModel (Element xsd:element)
//
// Sample instance
// <xsd:unique name="NCName">
//    <xsd:selector xpath="token">
//    </xsd:selector>
//    <xsd:field xpath="token">
//    </xsd:field>
// </xsd:unique>
#[element(name = "unique")]
pub struct Unique(pub KeyBase);

