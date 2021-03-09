use crate::model::groups::complex_type_model::ComplexTypeModel;

use crate::model::TopLevelComplexType;
use crate::xml_to_xsd::utils::annotation_first;
use roxmltree::Node;
use std::convert::TryInto;
use crate::model::attributes::abstract_::Abstract;
use crate::model::attributes::mixed::Mixed;

impl<'a> TopLevelComplexType<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let annotation = annotation_first(node)?;
        let model = ComplexTypeModel::parse(node)?;

        let mut attributes = vec![];
        let mut id = None;
        let mut name = None;
        let mut abstract_= Abstract::default();
        let mut final_ = None;
        let mut block = None;
        let mut mixed= Mixed::default() ;

        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.try_into()?),
                "name" => name = Some(attr.try_into()?),
                "abstract" => abstract_ = attr.try_into()?,
                "final" => final_ = Some(attr.try_into()?),
                "block" => block = Some(attr.try_into()?),
                "mixed" => mixed = attr.try_into()?,
                _ => attributes.push(attr.clone()),
            };
        }

        Ok(Self {
            annotation,
            model,
            attributes,
            id,
            name: name
                .ok_or_else(|| format!("Name required for xsd:topLevelComplexType: {:?}", node))?,
            abstract_,
            final_,
            block,
            mixed,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::model::groups::type_def_particle::TypeDefParticle;
    use crate::model::TopLevelComplexType;

    #[test]
    fn test_top_level_complex_type_parse() {
        let doc = roxmltree::Document::parse(
            r##"
	<complexType name="FloatRange" xmlns="http://www.w3.org/2001/XMLSchema" id="ID" a='a'>
		<annotation>
			<documentation>DocText</documentation>
		</annotation>
		<sequence>
			<element name="Min" type="xs:float"/>
			<element name="Max" type="xs:float"/>
		</sequence>
	</complexType>
                 "##,
        )
        .unwrap();
        let root = doc.root_element();
        let res = TopLevelComplexType::parse(root).unwrap();
        assert_eq!(res.annotation.as_ref().unwrap().doc_str(0), Some("DocText"));
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.id.as_ref().unwrap().0.as_ref(), "ID");
        assert_eq!(res.name.0.as_ref(), "FloatRange");
        if let TypeDefParticle::Sequence(val) = res.type_def_particle().unwrap() {
            assert_eq!(val.nested_particle.len(), 2);
        } else {
            panic!();
        }
    }
}
