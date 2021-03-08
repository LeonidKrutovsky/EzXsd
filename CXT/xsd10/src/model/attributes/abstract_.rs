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


#[attribute(name = "abstract")]
pub struct Abstract(Boolean);



#[cfg(test)]
mod test {
    use crate::model::attributes::abstract_::Abstract;

    #[test]
    pub fn test_name() {
        assert_eq!(Abstract::NAME, "abstract");
    }
}
