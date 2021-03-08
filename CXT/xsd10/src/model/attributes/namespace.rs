// namespace
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:namespaceList
// Properties: Local, Unqualified
//
// Value
//   Union of:
//     Type based on xsd:token
//         Valid value	    Description
//         ##any	        any non-conflicting replacement at all
//         ##other	        any non-conflicting replacement from a namespace other than target namespace
//     List of:
//         Union of:
//           xsd:anyURI
//           Type based on xsd:token
//             Valid value	                Description
//             ##targetNamespace	        any non-conflicting replacement from the target namespace
//             ##local	                    any unqualified (in no namespace) non-conflicting replacement
//
// Used in
// Anonymous type of element xsd:any via derivation of xsd:wildcard Type xsd:wildcard (Element xsd:anyAttribute)

use std::convert::TryFrom;
use crate::model::RawAttribute;
use crate::model::simple_types::namespace_list::NamespaceList;
use crate::model::simple_types::AnyUri;

#[derive(Debug, Default)]
pub struct Namespace(pub NamespaceList);

impl TryFrom<&RawAttribute<'_>> for Namespace {
    type Error = String;

    fn try_from(attr: &RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl Namespace {
    pub const NAME: &'static str = "namespace";
}




// namespace
// Namespace: None
// Schema documentation: xmlschema.xsd
// Type: xsd:anyURI
// Properties: Local, Unqualified
//
// Value:
// A value of type xsd anyURI
//
// Used in
// Anonymous type of element xsd:import

pub struct NamespaceUri(AnyUri);

impl TryFrom<RawAttribute<'_>> for NamespaceUri {
    type Error = String;

    fn try_from(attr: RawAttribute) -> Result<Self, Self::Error> {
        Ok(Self(attr.value().parse()?))
    }
}

impl NamespaceUri {
    pub const NAME: &'static str = "namespace";
}