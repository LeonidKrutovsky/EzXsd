use crate::model::attributes::{
    AttributeFormDefault, BlockDefault, ElementFormDefault, FinalDefault, Id, Language,
    RawAttribute, TargetNamespace, Version,
};
use crate::model::groups::SchemaTop;
use crate::model::{Annotation, Import, Include, Redefine};

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
#[derive(Debug, Default)]
pub struct Schema {
    pub includes: Vec<Include>,
    pub imports: Vec<Import>,
    pub redefines: Vec<Redefine>,
    pub annotations: Vec<Annotation>,
    pub content: Vec<(SchemaTop, Vec<Annotation>)>,

    pub attributes: Vec<RawAttribute>,
    pub target_namespace: Option<TargetNamespace>,
    pub version: Option<Version>,
    pub final_default: Option<FinalDefault>,
    pub block_default: Option<BlockDefault>,
    pub attribute_form_default: AttributeFormDefault,
    pub element_form_default: ElementFormDefault,
    pub id: Option<Id>,
    pub lang: Option<Language>,
}

impl Schema {
    pub fn parse(node: roxmltree::Node<'_, '_>) -> Result<Self, String> {
        let mut schema = Self::default();
        let mut annotations: Vec<Annotation> = vec![];
        let mut schema_top: Option<SchemaTop> = None;

        for ch in node.children().filter(|n| n.is_element()) {
            if schema_top.is_none() {
                match ch.tag_name().name() {
                    Include::NAME => schema.includes.push(Include::parse(ch)?),
                    Import::NAME => schema.imports.push(Import::parse(ch)?),
                    Redefine::NAME => schema.redefines.push(Redefine::parse(ch)?),
                    Annotation::NAME => schema.annotations.push(Annotation::parse(ch)?),
                    tag_name if SchemaTop::NAMES.contains(&tag_name) => {
                        schema_top = Some(SchemaTop::parse(ch)?)
                    }
                    _ => {}
                }
            } else {
                match ch.tag_name().name() {
                    Annotation::NAME => annotations.push(Annotation::parse(ch)?),
                    tag_name if SchemaTop::NAMES.contains(&tag_name) => {
                        schema.content.push((schema_top.unwrap(), annotations));
                        annotations = vec![];
                        schema_top = Some(SchemaTop::parse(ch)?);
                    }
                    _ => {}
                }
            }
        }

        for attr in node.attributes() {
                match attr.name() {
                    TargetNamespace::NAME => {
                        schema.target_namespace = Some(TargetNamespace::parse(attr)?)
                    }
                    Version::NAME => schema.version = Some(Version::parse(attr)?),
                    FinalDefault::NAME => schema.final_default = Some(FinalDefault::parse(attr)?),
                    BlockDefault::NAME => schema.block_default = Some(BlockDefault::parse(attr)?),
                    AttributeFormDefault::NAME => {
                        schema.attribute_form_default = AttributeFormDefault::parse(attr)?
                    }
                    ElementFormDefault::NAME => {
                        schema.element_form_default = ElementFormDefault::parse(attr)?
                    }
                    Id::NAME => schema.id = Some(Id::parse(attr)?),
                    Language::NAME => schema.lang = Some(Language::parse(attr)?),
                    _ => schema.attributes.push(RawAttribute::parse(attr)?),
                }
            }
        Ok(schema)
    }
}
