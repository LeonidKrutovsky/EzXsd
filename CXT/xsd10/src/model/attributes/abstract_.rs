// abstract
// Attribute information
// Namespace: None
// Schema document: xmlschema.xsd
// Type: xsd:boolean
// Properties: Local, Unqualified
//
// Value
// A value of type xsd:boolean.

// Used in
// Type xsd:topLevelComplexType (Element xsd:complexType)
// Type xsd:topLevelElement (Element xsd:element)

use crate::model::simple_types::Boolean;
use crate::model::RawAttribute;
use std::convert::TryFrom;
extern crate xml_utils;
use xml_utils::*;

//#[derive(Attribute)]
#[attribute("abstract")]
pub struct Abstract(Boolean);

impl Abstract {
    const NAME: &'static str = "abstract";
}

impl TryFrom<RawAttribute<'_>> for Abstract {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

#[cfg(test)]
mod test {
    use crate::model::attributes::abstract_::Abstract;

    #[test]
    pub fn test_name() {
        //Abstract::describe();
        assert_eq!(Abstract::NAME, "abstract");
    }
}
