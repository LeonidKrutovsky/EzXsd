// xsd:IDREF
// The type xsd:IDREF is used for an attribute that references an ID. All attributes of type xsd:IDREF must reference an xsd:ID in the same XML document. A common use case for xsd:IDREF is to create a cross-reference to a particular section of a document. Like ID, an xsd:IDREF value must be an NCName.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:NCName
// Pattern: [\i-[:]][\c-[:]]* (Defined in type xsd:NCName)
// White Space: collapse (Defined in type xsd:token)
// Type Inheritance Chain
//  xsd:anySimpleType
//    restricted by xsd:string
//      restricted by xsd:normalizedString
//        restricted by xsd:token
//          restricted by xsd:Name
//            restricted by xsd:NCName
//              restricted by xsd:IDREF
//                used in list xsd:IDREFS

use crate::model::simple_types::NCName;

pub type IdRef = NCName;
