// xsd:QName
// The type xsd:QName represents an XML namespace-qualified name.
// A xsd:QName value consists of a prefix and a local part, separated by a colon,
// both of which are NCName values. The prefix and colon are optional,
// but if they are not present, it is assumed that either the name is namespace-qualified
// by other means (e.g., by a default namespace declaration), or the name is not in a namespace.
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse
// Examples
// Valid values	        Comment
// pre:myElement	        valid assuming the prefix "pre" is mapped to a namespace in scope
// myElement	            prefix and colon are optional
// Invalid values	    Comment
// :myElement	            a QName must not start with a colon
// pre:3rdElement	        the local part must not start with a number; it must be a valid NCName
//                          an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:QName

use crate::model::{Parse};
use core::fmt;
use crate::model::simple_types::NCName;

#[derive(Default, Debug)]
pub struct QName {
    prefix: Option<NCName>,
    name: NCName
}

impl Parse for QName {
    fn parse(value: &str) -> Result<Self, String> where Self: Sized {
        if let Some(index) = value.find(':') {
            Ok(Self{
                prefix: Some(value[0..index].parse()?),
                name: value[index+1..].parse()?
            })
        } else {
            Ok(Self{
                prefix: None,
                name: value.parse()?
            })
        }
    }

    fn create(value: String) -> Self where Self: Sized {
        if let Some(index) = value.find(':') {
            Self{
                prefix: Some(value[0..index].to_string().into()),
                name: value[index+1..].to_string().into()
            }
        } else {
            Self{
                prefix: None,
                name: value.into()
            }
        }
    }

    fn text(&self) -> Result<String, String> {
        if let Some(ref pref) = self.prefix {
            Ok(format!("{}:{}", pref.text()?, self.name.text()?))
        } else {
            self.name.text()
        }
    }
}

impl_from_str!(QName);
impl_from_string!(QName);

impl QName {
    pub fn prefix(&self) -> Option<&str> {
        if let Some(ref pref) = self.prefix {
            Some(pref.raw())
        } else {
            None
        }
    }

    pub fn name(&self) -> &str {
        self.name.raw()
    }
}


impl fmt::Display for QName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(prefix) = self.prefix() {
            write!(f, "{}:{}", prefix, self.name())
        } else {
            write!(f, "{}", self.name())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::simple_types::qname::QName;
    use crate::model::Parse;

    #[test]
    fn test_valid_qname() {
        fn eq(left: &str, right: &str) {
            assert_eq!(QName::create(left.to_string()).text().unwrap(), right);
        }

        eq("pre:myElement", "pre:myElement");
        eq("myElement", "myElement");
        eq("  myElement  ", "myElement");
    }

    #[test]
    fn test_invalid_qname() {
        assert_eq!(
            QName::create(":myElement".to_string()).text(),
            Err("A QName must not start with a colon: :myElement".to_string())
        );

        assert_eq!(
        QName::create("pre:3rdElement".to_string()).text(),
        Err("The local part must not start with a number; it must be a valid NCName: pre:3rdElement".to_string())
        );
    }
}
