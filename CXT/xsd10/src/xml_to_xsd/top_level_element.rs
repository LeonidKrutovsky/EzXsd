use crate::model::groups::element_model::ElementModel;
use crate::model::TopLevelElement;
use crate::xml_to_xsd::utils::annotation_first;
use roxmltree::Node;
use std::convert::TryInto;
use crate::model::attributes::id::Id;
use crate::model::attributes::name::Name;
use crate::model::attributes::type_::Type;
use crate::model::attributes::substitution_group::SubstitutionGroup;
use crate::model::attributes::default::Default_;
use crate::model::attributes::fixed::Fixed;
use crate::model::attributes::nillable::Nillable;
use crate::model::attributes::abstract_::Abstract;
use crate::model::attributes::final_::Final;
use crate::model::attributes::block::Block;

// Attributes
// Any attribute	    [0..*]		                Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	                [0..1]	xsd:ID		                                                    from type xsd:annotated
// name	                [1..1]	xsd:NCName
// type	                [0..1]	xsd:QName
// substitutionGroup	[0..1]	xsd:QName
// default	            [0..1]	xsd:string
// fixed	            [0..1]	xsd:string
// nillable	            [0..1]	xsd:boolean		    Default value is "false".
// abstract	            [0..1]	xsd:boolean		    Default value is "false".
// final	            [0..1]	xsd:derivationSet
// block	            [0..1]	xsd:blockSet
impl<'a> TopLevelElement<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let annotation = annotation_first(node)?;
        let model = ElementModel::parse(node)?;
        let mut attributes = vec![];
        let mut id = None;
        let mut name = None;
        let mut type_ = None;
        let mut substitution_group = None;
        let mut default = None;
        let mut fixed = None;
        let mut nillable = Default::default();
        let mut abstract_ = Default::default();
        let mut final_ = None;
        let mut block = None;

        for attr in node.attributes() {
            match attr.name() {
                Id::NAME => id = Some(attr.try_into()?),
                Name::NAME => name = Some(attr.try_into()?),
                Type::NAME => type_ = Some(attr.try_into()?),
                SubstitutionGroup::NAME => substitution_group = Some(attr.try_into()?),
                Default_::NAME => default = Some(attr.try_into()?),
                Fixed::NAME => fixed = Some(attr.try_into()?),
                Nillable::NAME => nillable = attr.try_into()?,
                Abstract::NAME => abstract_ = attr.try_into()?,
                Final::NAME => final_ = Some(attr.try_into()?),
                Block::NAME => block = Some(attr.try_into()?),

                _ => attributes.push(attr.clone()),
            };
        }

        let name = name.ok_or_else(|| format!("Name required for xsd:topLevelElement: {:?}", node))?;

        Ok(Self {
            annotation,
            model,
            attributes,
            id,
            name,
            type_,
            substitution_group,
            default,
            fixed,
            nillable,
            abstract_,
            final_,
            block,
        })
    }
}
