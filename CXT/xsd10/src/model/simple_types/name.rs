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

use crate::model::simple_types::Token;
use crate::model::Parse;

#[derive(Debug, Default, Clone)]
pub struct Name(Token);

impl Parse for Name {
    fn parse(value: &str) -> Result<Self, String> {
        if value.is_empty() {
            Err(format!(
                "An empty value is not valid, unless xsi:nil is used"
            ))
        } else if value.starts_with('-') {
            Err(format!("A Name must not start with a hyphen: {}", value))
        } else if value.chars().nth(0).unwrap().is_digit(10) {
            Err(format!("A Name must not start with a number: {}", value))
        } else {
            Ok(Self(Token::parse(value)?))
        }
    }

    fn create(value: String) -> Self {
        Self(Token::create(value))
    }

    fn text(&self) -> Result<String, String> {
        let result: String = self.0.text()?;
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

impl Name {
    pub fn raw(&self) -> &str {
        self.0.raw()
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::name::Name;
    use crate::model::Parse;

    #[test]
    fn test_valid_name() {
        fn eq(left: &str, right: &str) {
            assert_eq!(Name::create(left.to_string()).text().unwrap(), right);
        }

        eq("myElement", "myElement");
        eq("_my.Element", "_my.Element");
        eq("my-element", "my-element");
        eq("pre:myelement3", "pre:myelement3");
    }

    #[test]
    fn test_invalid_name() {
        assert_eq!(
            Name::create("-myelement".to_string()).text(),
            Err("A Name must not start with a hyphen: -myelement".to_string())
        );

        assert_eq!(
            Name::create("3myelement".to_string()).text(),
            Err("A Name must not start with a number: 3myelement".to_string())
        );

        assert_eq!(
            Name::create(String::new()).text(),
            Err("An empty value is not valid, unless xsi:nil is used".to_string())
        );
    }
}
