// xsd:NCName
// The type xsd:NCName represents an XML non-colonized name, which is simply a name that
// does not contain colons. An xsd:NCName value must start with either a letter or
// underscore (_) and may contain only letters, digits, underscores (_), hyphens (-),
// and periods (.). This is equivalent to the Name type, except that colons are not permitted.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:Name
// Pattern: [\i-[:]][\c-[:]]*
// White Space: collapse (Defined in type xsd:token)
//
// Examples
// Valid values	        Comment
// myElement
// _my.Element
// my-element
// Invalid values	    Comment
// pre:myElement	    an NCName must not contain a colon
// -myelement	        an NCName must not start with a hyphen
//                      an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:Name
//                      restricted by xsd:NCName
//                          restricted by xsd:ID
//                              restricted by xsd:IDREF
//                                  used in list xsd:IDREFS
//                              restricted by xsd:ENTITY
//                                  used in list xsd:ENTITIES

use std::str::FromStr;

use crate::model::simple_types::name::Name;

#[derive(Debug, Default, Clone)]
pub struct NCName(Name);

impl FromStr for NCName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(":") {
            Err(format!("An NCName must not contain a colon: {}", s))
        } else {
            Ok(Self(s.parse()?))
        }
    }
}

impl_from_string!(NCName);
impl_as_ref!(NCName);
impl_display!(NCName);


#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::model::simple_types::ncname::NCName;

    #[test]
    fn test_valid_name() {
        fn is_ok(s: &str) {
            assert!(NCName::from_str(s).is_ok());
        }

        is_ok("myElement");
        is_ok("_my.Element");
        is_ok("my-element");
    }

    #[test]
    fn test_invalid_name() {
        assert_eq!(
            NCName::from_str("-myelement").err().unwrap(),
            "A Name must not start with a hyphen: -myelement"
        );

        assert_eq!(
            NCName::from_str("3myelement").err().unwrap(),
            "A Name must not start with a number: 3myelement"
        );

        assert_eq!(
            NCName::from_str("").err().unwrap(),
            "An empty value is not valid, unless xsi:nil is used"
        );

        assert_eq!(
            NCName::from_str("pre:myElement").err().unwrap(),
            "An NCName must not contain a colon: pre:myElement"
        );
    }
}
