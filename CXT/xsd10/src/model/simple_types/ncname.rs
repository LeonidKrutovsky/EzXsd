// xsd:NCName
// The type xsd:NCName represents an XML non-colonized name, which is simply a name that does not contain colons. An xsd:NCName value must start with either a letter or underscore (_) and may contain only letters, digits, underscores (_), hyphens (-), and periods (.). This is equivalent to the Name type, except that colons are not permitted.
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

use crate::model::simple_types::name::Name;
use crate::model::ToXml;
use std::borrow::Cow;

#[derive(Debug, Default, Clone)]
pub struct NCName<'a>(pub &'a str);

#[derive(Debug)]
pub struct NCName_<'a>(Name<'a>);

impl<'a, T> From<T> for NCName_<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self {
            0: Name::from(value),
        }
    }
}

impl<'a> ToXml for NCName_<'a> {
    fn to_xml(&self) -> Result<String, String> {
        let result = self.0.to_xml()?;
        if result.contains(":") {
            Err(format!("An NCName must not contain a colon: {}", result))
        } else {
            Ok(result)
        }
    }

    fn raw(&self) -> &str {
        self.0.raw()
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::ncname::NCName_;
    use crate::model::ToXml;

    #[test]
    fn test_valid_name() {
        fn eq(left: &str, right: &str) {
            assert_eq!(NCName_::from(left).to_xml().unwrap(), right);
        }

        eq("myElement", "myElement");
        eq("_my.Element", "_my.Element");
        eq("my-element", "my-element");
    }

    #[test]
    fn test_invalid_name() {
        assert_eq!(
            NCName_::from("-myelement").to_xml(),
            Err("A Name must not start with a hyphen: -myelement".to_string())
        );

        assert_eq!(
            NCName_::from("3myelement").to_xml(),
            Err("A Name must not start with a number: 3myelement".to_string())
        );

        assert_eq!(
            NCName_::from("").to_xml(),
            Err("An empty value is not valid, unless xsi:nil is used".to_string())
        );

        assert_eq!(
            NCName_::from("pre:myElement").to_xml(),
            Err("An NCName must not contain a colon: pre:myElement".to_string())
        );
    }
}
