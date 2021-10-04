use crate::model::elements::complex_content::ComplexContent;
use crate::model::elements::simple_content::SimpleContent;
use crate::model::groups::attr_decls::AttrDecls;
use crate::model::groups::type_def_particle::TypeDefParticle;
use crate::model::elements;

// xsd:complexTypeModel
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      xsd:simpleContent
//      xsd:complexContent
//      Sequence [1..1]
//          Choice [0..1]       from group xsd:typeDefParticle
//              xsd:group
//              xsd:all         An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//              xsd:choice
//              xsd:sequence
//          Choice [0..*]       from group xsd:attrDecls
//              xsd:attribute
//              xsd:attributeGroup
//          xsd:anyAttribute [0..1]
//
// Used in
// Type xsd:complexType
// Type xsd:localComplexType (Element xsd:complexType)
// Type xsd:topLevelComplexType (Element xsd:complexType)
#[derive(Debug)]
pub enum ComplexTypeModel {
    SimpleContent(SimpleContent),
    ComplexContent(ComplexContent),
    Content(Option<TypeDefParticle>, Box<AttrDecls>),
}

impl Default for ComplexTypeModel {
    fn default() -> Self {
        Self::Content(None, Box::new(AttrDecls::default()))
    }
}

impl ComplexTypeModel {
    pub const NAMES: &'static [&'static str] = &[
        elements::SimpleContent::NAME,
        elements::ComplexContent::NAME,

        elements::Group::NAME,
        elements::AllType::NAME,
        elements::Choice::NAME,
        elements::Sequence::NAME,

        elements::LocalAttribute::NAME,
        elements::AttributeGroupRef::NAME,
        elements::AnyAttribute::NAME,
    ];

    pub fn push(&mut self, node: roxmltree::Node<'_, '_>) -> Result<(), String> {
        match self {
            ComplexTypeModel::SimpleContent(_) | ComplexTypeModel::ComplexContent(_) => {
                Err(format!("Unexpected node in ComplexTypeModel group: {:?}", node))?
            }
            ComplexTypeModel::Content(tdp, ad) => {
                if tdp.is_some() || !ad.is_empty() {
                    match node.tag_name().name() {
                        SimpleContent::NAME | ComplexContent::NAME => {
                            Err(format!("Unexpected node in ComplexTypeModel group: {:?}", node))?
                        }
                        tag_name if TypeDefParticle::NAMES.contains(&tag_name) => {
                            tdp.insert(TypeDefParticle::parse(node)?);
                        }
                        tag_name if AttrDecls::NAMES.contains(&tag_name) => {
                            ad.push(node);
                        }
                        _ => {}
                    }
                } else {
                    match node.tag_name().name() {
                        SimpleContent::NAME => {*self = Self::SimpleContent(SimpleContent::parse(node)?)}
                        ComplexContent::NAME => {*self = Self::ComplexContent(ComplexContent::parse(node)?)}
                        tag_name if TypeDefParticle::NAMES.contains(&tag_name) => {
                            tdp.insert(TypeDefParticle::parse(node)?);
                        }
                        tag_name if AttrDecls::NAMES.contains(&tag_name) => {
                            ad.push(node)?;
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}
