// xsd:Name
// The type xsd:Name represents an XML name, which can be used as an element-type
// name or attribute name, among other things. Specifically, this means that values
// must start with a letter, underscore(_), or colon (:), and may contain only letters,
// digits, underscores (_), colons (:), hyphens (-), and periods (.).
// Colons should only be used to separate namespace prefixes from local names.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:token
// Pattern: \i\c*
// White Space: collapse (Defined in type xsd:token)

// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:Name
//                      restricted by xsd:NCName
//                          restricted by xsd:ID
//                          restricted by xsd:IDREF
//                              used in list xsd:IDREFS
//                          restricted by xsd:ENTITY
//                              used in list xsd:ENTITIES

use crate::model::simple_types::Token_;
use crate::model::ToXml;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Name<'a>(Token_<'a>);

impl<'a, T> From<T> for Name<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self {
            0: Token_::from(value),
        }
    }
}

impl<'a> ToXml for Name<'a> {
    fn to_xml(&self) -> Result<String, String> {
        let result = self.0.to_xml()?;
        if result.is_empty() {
            Err(format!(
                "An empty value is not valid, unless xsi:nil is used"
            ))
        } else if result.starts_with('-') {
            Err(format!("A Name must not start with a hyphen: {}", result))
        } else if result.chars().nth(0).unwrap().is_digit(10) {
            Err(format!("A Name must not start with a number: {}", result))
        } else {
            Ok(result)
        }
    }
}

impl<'a> Name<'a> {
    pub fn raw(&self) -> &str {
        self.0.raw()
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::name::Name;
    use crate::model::ToXml;

    #[test]
    fn test_valid_name() {
        fn eq(left: &str, right: &str) {
            assert_eq!(Name::from(left).to_xml().unwrap(), right);
        }

        eq("myElement", "myElement");
        eq("_my.Element", "_my.Element");
        eq("my-element", "my-element");
        eq("pre:myelement3", "pre:myelement3");
    }

    #[test]
    fn test_invalid_name() {
        assert_eq!(
            Name::from("-myelement").to_xml(),
            Err("A Name must not start with a hyphen: -myelement".to_string())
        );

        assert_eq!(
            Name::from("3myelement").to_xml(),
            Err("A Name must not start with a number: 3myelement".to_string())
        );

        assert_eq!(
            Name::from("").to_xml(),
            Err("An empty value is not valid, unless xsi:nil is used".to_string())
        );
    }
}
