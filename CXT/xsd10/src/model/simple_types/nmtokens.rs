// xsd:NMTOKENS
// The type xsd:NMTOKENS represents a list of NMTOKEN values separated by whitespace.
// There must be at least one NMTOKEN in the list.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:NMTOKEN
// Minimum Length: 1
// White Space: collapse

// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:language
//                  restricted by xsd:NMTOKEN
//                      used in list xsd:NMTOKENS

use crate::model::simple_types::nmtoken::NmToken;
use crate::model::simple_types::xsd_list::NotEmptyXsdList;

pub type NmTokens = NotEmptyXsdList<NmToken>;
