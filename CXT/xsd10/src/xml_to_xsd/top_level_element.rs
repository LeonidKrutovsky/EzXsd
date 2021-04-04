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
use crate::model::attributes::AnyAttributes;


impl TopLevelElement {
    pub fn parse(node: Node<'_, '_>) -> Result<Self, String> {
        let annotation = annotation_first(node)?;
        let model = ElementModel::parse(node)?;
        let mut attributes= AnyAttributes::default();
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

                _ => attributes.push(attr.try_into()?),
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

#[cfg(test)]
mod test {
    use crate::model::TopLevelElement;

    #[test]
    pub fn test_parse() {
        let xsd = r##"<element
        name="GetAccessProfileInfo"
        id="id42"
        substitutionGroup="ns:Name"
        default="Default"
        fixed="Fixed"
        nillable="true"
        abstract="true"
        final="extension"
        block="#all"
        >
                    <complexType>
                        <sequence>
                            <element name="Token" type="pt:ReferenceToken" minOccurs="1" maxOccurs="unbounded">
                                <annotation>
                                    <documentation>Tokens of AccessProfileInfo items to get.</documentation>
                                </annotation>
                            </element>
                        </sequence>
                    </complexType>
                </element>"##;

        let doc = roxmltree::Document::parse(xsd).unwrap();
        let root = doc.root_element();
        let res = TopLevelElement::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.id.unwrap().0.as_ref(), "id42");
        assert_eq!(res.substitution_group.unwrap().0.to_string(), "ns:Name");
        assert_eq!(res.default.unwrap().0.as_ref(), "Default");
        assert_eq!(res.fixed.unwrap().0.as_ref(), "Fixed");
        assert_eq!(res.nillable.0, true);
        assert_eq!(res.abstract_.0, true);
        assert_eq!(res.final_.unwrap().0.to_string(), "extension");
        assert_eq!(res.block.unwrap().0.to_string(), "#all");
    }
}
