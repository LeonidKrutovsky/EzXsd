use crate::model::elements::annotation::Annotation;
use crate::model::elements::import::Import;
use crate::model::elements::include::Include;
use crate::model::elements::redefine::Redefine;
use crate::model::groups::schema_top::SchemaTop;
use crate::model::attributes::id::Id;
use crate::model::simple_types::Language;
use crate::model::attributes::target_namespace::TargetNamespace;
use crate::model::attributes::version::Version;
use crate::model::attributes::final_default::FinalDefault;
use crate::model::attributes::block_default::BlockDefault;
use crate::model::attributes::attribute_form_default::AttributeFormDefault;
use crate::model::attributes::element_form_default::ElementFormDefault;
use crate::model::attributes::AnyAttributes;

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
#[derive(Default, Debug)]
pub struct Schema {
    pub includes: Vec<Include>,
    pub imports: Vec<Import>,
    pub redefines: Vec<Redefine>,
    pub annotations: Vec<Annotation>,
    pub content: Vec<(SchemaTop, Vec<Annotation>)>,
    pub attributes: AnyAttributes,
    pub target_namespace: Option<TargetNamespace>,
    pub version: Option<Version>,
    pub final_default: FinalDefault,
    pub block_default: BlockDefault,
    pub attribute_form_default: AttributeFormDefault,
    pub element_form_default: ElementFormDefault,
    pub id: Option<Id>,
    pub lang: Option<Language>,
}
