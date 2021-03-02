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

use crate::model::simple_types::Token;
use crate::model::Parse;

#[derive(Debug)]
pub struct NmToken(Token);

impl Parse for NmToken {
    fn parse(value: &str) -> Result<Self, String> {
        if value.is_empty() {
            Err(format!(
                "An empty value is not valid NMTOKEN, unless xsi:nil is used"
            ))
        } else if value.contains(" ") {
            Err(format!("NMTOKEN must not contain a space: {}", value))
        } else {
            Ok(Self(Token::parse(value)?))
        }
    }

    fn create(value: String) -> Self {
        Self(Token::create(value))
    }

    fn text(&self) -> Result<String, String> {
        let result = self.0.text()?;
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
}

impl NmToken {
    pub fn raw(&self) -> &str {
        self.0.raw()
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::nmtoken::NmToken;
    use crate::model::Parse;

    #[test]
    fn test_valid_string() {
        fn eq(left: &str, right: &str) {
            assert_eq!(NmToken::create(left.to_string()).text().unwrap(), right);
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
            NmToken::create("name with spaces".to_string()).text(),
            Err("NmToken must not contain a space: name with spaces".to_string())
        );

        assert_eq!(
            NmToken::create("".to_string()).text(),
            Err("An empty value is not valid, unless xsi:nil is used".to_string())
        );
    }
}
