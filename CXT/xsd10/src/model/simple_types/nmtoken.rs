// xsd:NMTOKEN
// The type xsd:NMTOKEN represents a single string token. xsd:NMTOKEN values
// may consist of letters, digits, periods (.), hyphens (-), underscores (_), and colons (:).
// They may start with any of these characters. xsd:NMTOKEN has a whiteSpace facet
// value of collapse, so any leading or trailing whitespace will be removed.
// However, no whitespace may appear within the value itself.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:token
// Pattern: \c+
// White Space: collapse (Defined in type xsd:token)

// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:language
//                  restricted by xsd:NMTOKEN
//                      used in list xsd:NMTOKENS

use crate::model::simple_types::Token_;
use crate::model::ToXml;
use std::borrow::Cow;

#[derive(Debug)]
pub struct NmToken<'a>(Token_<'a>);

impl<'a, T> From<T> for NmToken<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self {
            0: Token_::from(value),
        }
    }
}

impl<'a> ToXml for NmToken<'a> {
    fn to_xml(&self) -> Result<String, String> {
        let result = self.0.to_xml()?;
        if result.is_empty() {
            Err(format!(
                "An empty value is not valid, unless xsi:nil is used"
            ))
        } else if result.contains(" ") {
            Err(format!("NmToken must not contain a space: {}", result))
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
    use crate::model::simple_types::nmtoken::NmToken as Str;
    use crate::model::ToXml;

    #[test]
    fn test_valid_string() {
        fn eq(left: &str, right: &str) {
            assert_eq!(Str::from(left).to_xml().unwrap(), right);
        }
        eq("12.5", "12.5");
        eq("   trim_spaces.   ", "trim_spaces.");
        eq("3<4", "3&lt;4");
        eq("AT&T", "AT&amp;T");
        eq("  WS:    ", "WS:");
    }

    #[test]
    fn test_err_string() {
        assert_eq!(
            Str::from("name with spaces").to_xml(),
            Err("NmToken must not contain a space: name with spaces".to_string())
        );

        assert_eq!(
            Str::from("").to_xml(),
            Err("An empty value is not valid, unless xsi:nil is used".to_string())
        );
    }
}
