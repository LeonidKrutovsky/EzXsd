// xsd:base64Binary
// The type xsd:base64Binary represents binary data as a sequence of binary octets.
// It uses base64 encoding, as described in RFC 2045.The following rules apply to
// xsd:base64Binary values:
// The following characters are allowed: the letters A to Z (upper and lower case),
// digits 0 through 9, the plus sign ("+"), the slash ("/"), the equals sign ("=")
// and XML whitespace characters.
// XML whitespace characters may appear anywhere in the value.
// The number of non-whitespace characters must be divisible by 4.
// Equals signs may only appear at the end of the value, and there may be zero,
// one or two of them. If there are two equals signs, they must be preceded by one
// of the following characters: AQgw. If there is only one equals sign, it must be preceded
// by one of the following characters: AEIMQUYcgkosw048. In either case, there may
// be whitespace in between the necessary characters and the equals sign(s).
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse
//
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:base64Binary

use crate::model::simple_types::white_space_facet::collapse;
use crate::model::ToXml;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Base64Binary(Vec<u8>);

impl FromStr for Base64Binary {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(' ', "");
        if s.len() % 2 != 0 {
            Err(format!("An odd number of characters is not valid; \
            characters appear in groups of four: {}", s))
        } else {
            Ok(Self(base64::decode(s).map_err(|e|e.to_string())?))
        }
    }
}

impl Base64Binary {
    pub fn decoded(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn encoded(&self) -> String {
        base64::encode(self.0.as_slice())
    }
}

impl ToXml for Base64Binary {
    fn to_xml(&self) -> Result<String, String> {
        Ok(collapse(self.encoded().as_str()))
    }

    fn raw(&self) -> &str {
        unimplemented!()
    }
}


#[cfg(test)]
mod test {
    use crate::model::simple_types::Base64Binary;
    use std::str::FromStr;

    #[test]
    pub fn test_decode() {
        fn eq(left: &str, right: &[u8]) {
            assert_eq!(Base64Binary::from_str(left).unwrap().decoded(), right);
        }
        eq("0FB8", &[208, 80, 124]);
        eq("0fb8", &[209, 246, 252]);
        eq("", &[]);
        eq("0 FB8 0F+9", &[208, 80, 124, 208, 95, 189]);
        eq("0F+40A==", &[208, 95, 184, 208]);
    }

        #[test]
    pub fn test_encode() {
        fn eq(left: &str, right: &str) {
            assert_eq!(Base64Binary::from_str(left).unwrap().encoded(), right);
        }
        eq("0FB8", "0FB8");
        eq("0fb8", "0fb8");
        eq("", "");
        eq("0 FB8  0F+9", "0FB80F+9");
        eq("0F+40A==", "0F+40A==");

    }

    #[test]
    fn test_err() {
                assert!(Base64Binary::from_str("FB8").is_err());
                assert!(Base64Binary::from_str("==0F").is_err());
            }
}
