use xml_utils::*;

use crate::model::simple_types::QName;

// base
// Attribute information
// Namespace: None
//
// Schema document: datatypes.xsd
//
// Type: xsd:QName
//
// Properties: Local, Unqualified
//
// Value
// A value of type xsd:QName.
// Used in
// Anonymous type of element xsd:restriction
// Type xsd:complexRestrictionType (Element xsd:restriction)
// Type xsd:extensionType (Element xsd:extension)
// Type xsd:simpleExtensionType (Element xsd:extension)
// Type xsd:simpleRestrictionType (Element xsd:restriction)
#[attribute(name = "base")]
pub struct Base(pub QName);

#[cfg(test)]
mod tests {
    use crate::model::attributes::Base;

    #[test]
    fn text() {
        let base = Base("xss:SomeName".parse().unwrap());
        assert_eq!(base.text(), " base=\"xss:SomeName\"")
    }
}
