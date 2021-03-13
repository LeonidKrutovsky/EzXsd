use crate::model::elements::annotation::Annotation;
use crate::model::groups::attr_decls::AttrDecls;
use crate::model::groups::complex_type_model::ComplexTypeModel;
use crate::model::groups::type_def_particle::TypeDefParticle;
use crate::model::{ComplexContent, SimpleContent};
use crate::model::attributes::name::Name;
use crate::model::attributes::id::Id;
use crate::model::attributes::final_::Final;
use crate::model::attributes::block::DerivationBlock;
use crate::model::attributes::mixed::Mixed;
use crate::model::attributes::abstract_::Abstract;
use crate::model::attributes::AnyAttributes;

// xsd:topLevelComplexType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      Choice [1..1]    from group xsd:complexTypeModel
//          xsd:simpleContent
//          xsd:complexContent
//          Sequence [1..1]
//              Choice [0..1]    from group xsd:typeDefParticle
//                  xsd:group
//                  xsd:all    An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//                  xsd:choice
//                  xsd:sequence
//              Choice [0..*]    from group xsd:attrDecls
//                  xsd:attribute
//                  xsd:attributeGroup
//              xsd:anyAttribute [0..1]
//
// Attributes
// Any attribute	[0..*]		                Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                    from type xsd:annotated
// name	            [1..1]	xsd:NCName
// abstract	        [0..1]	xsd:boolean		                                                Default value is "false".
// final	        [0..1]	xsd:derivationSet
// block	        [0..1]	xsd:derivationSet
// mixed	        [0..1]	xsd:boolean	        Indicates that mixed content is allowed.	Default value is "false".
//
// Used by
// Element xsd:complexType
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:topLevelComplexType
#[derive(Debug)]
pub struct TopLevelComplexType<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub model: ComplexTypeModel<'a>,
    pub attributes: AnyAttributes,
    pub id: Option<Id>,
    pub name: Name,
    pub abstract_: Abstract,
    pub final_: Option<Final>,
    pub block: Option<DerivationBlock>,
    pub mixed: Mixed,
}

impl<'a> TopLevelComplexType<'a> {
    pub fn simple_content(&self) -> Option<&SimpleContent<'a>> {
        match &self.model {
            ComplexTypeModel::SimpleContent(sc) => Some(sc),
            _ => None,
        }
    }

    pub fn complex_content(&self) -> Option<&ComplexContent<'a>> {
        match &self.model {
            ComplexTypeModel::ComplexContent(sc) => Some(sc),
            _ => None,
        }
    }

    pub fn type_def_particle(&self) -> Option<&TypeDefParticle<'a>> {
        match &self.model {
            ComplexTypeModel::Content(tdp, _) => tdp.as_ref(),
            _ => None,
        }
    }

    pub fn attr_decls(&self) -> Option<&AttrDecls<'a>> {
        match &self.model {
            ComplexTypeModel::Content(_, attr_decls) => Some(attr_decls),
            _ => None,
        }
    }
}
