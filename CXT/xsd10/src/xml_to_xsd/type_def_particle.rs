use crate::model::complex_types::explicit_group::ExplicitGroup;
use crate::model::complex_types::named_group::NamedGroup;
use crate::model::elements::ElementType;
use crate::model::groups::type_def_particle::TypeDefParticle;
use roxmltree::Node;

impl TypeDefParticle {
    pub fn parse(node: Node<'_, '_>, element_type: ElementType) -> Result<Option<Self>, String> {
        Ok(Some(match element_type {
            ElementType::Group => TypeDefParticle::Group(NamedGroup::parse(node)?),
            ElementType::All => unimplemented!("Not presented in ONVIF"), //TypeDefParticle::All(),
            ElementType::Choice => TypeDefParticle::Choice(ExplicitGroup::parse(node)?),
            ElementType::Sequence => TypeDefParticle::Sequence(ExplicitGroup::parse(node)?),
            _ => return Ok(None),
        }))
    }
}
