use crate::model::elements;
use crate::model::groups::schema_top::SchemaTop;
use crate::model::attributes;
use xml_utils::element;

// xsd:schema
// See http://www.w3.org/TR/xmlschema-1/#element-schema.
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      Choice [0..*]
//          xsd:include
//          xsd:import
//          xsd:redefine
//          xsd:annotation
//      Sequence [0..*]
//          Choice [1..1]   from group xsd:schemaTop
//                          from group xsd:redefinable
//              xsd:simpleType
//              xsd:complexType
//              xsd:group
//              xsd:attributeGroup
//
//              xsd:element
//              xsd:attribute
//              xsd:notation
//          xsd:annotation [0..*]
//
// Attributes
// Any attribute	    [0..*]		                    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// targetNamespace	    [0..1]	xsd:anyURI
// version	            [0..1]	xsd:token
// finalDefault	        [0..1]	xsd:fullDerivationSet	Default value is "".
// blockDefault	        [0..1]	xsd:blockSet		    Default value is "".
// attributeFormDefault	[0..1]	xsd:formChoice		    Default value is "unqualified".
// elementFormDefault	[0..1]	xsd:formChoice		    Default value is "unqualified".
// id	                [0..1]	xsd:ID
// xml:lang	            [0..1]	Anonymous
//
// Identity constraints
// Type	Name	            Selector	                        Field(s)
// key	element	            xs:element	                        @name
// key	attribute	        xs:attribute	                    @name
// key	type	            xs:complexType|xs:simpleType        @name
// key	group	            xs:group	                        @name
// key	attributeGroup 	    xs:attributeGroup	                @name
// key	notation	        xs:notation	                        @name
// key	identityConstraint	.//xs:key|.//xs:unique|.//xs:keyref	@name
#[element(name = "schema")]
pub struct Schema {
    pub includes: Vec<elements::Include>,
    pub imports: Vec<elements::Import>,
    pub redefines: Vec<elements::Redefine>,
    pub annotations: Vec<elements::Annotation>,
    pub content: Vec<(SchemaTop, Vec<elements::Annotation>)>,
    pub attributes: attributes::AnyAttributes,
    pub target_namespace: Option<attributes::TargetNamespace>,
    pub version: Option<attributes::Version>,
    pub final_default: attributes::FinalDefault,
    pub block_default: attributes::BlockDefault,
    pub attribute_form_default: attributes::AttributeFormDefault,
    pub element_form_default: attributes::ElementFormDefault,
    pub id: Option<attributes::Id>,
    pub lang: Option<attributes::Language>,
}
