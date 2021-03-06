// substitutionGroup
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:QName
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd QName
//
// Used in
// Type xsd:topLevelElement (Element xsd:element)
//
use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::QName;

pub struct SubstitutionGroup(QName);

impl TryFrom<RawAttribute<'_>> for SubstitutionGroup {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl SubstitutionGroup {
    pub const NAME: &'static str = "substitutionGroup";
}