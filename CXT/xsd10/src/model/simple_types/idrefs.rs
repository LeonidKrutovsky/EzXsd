// xsd:IDREFS
// The type xsd:IDREFS represents a list of IDREF values separated by whitespace.
// There must be at least one IDREF in the list. Each of the IDREF values must
// match an ID contained in the same XML document.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:IDREF
// Minimum Length: 1
// White Space: collapse
// Type Inheritance Chain
//  xsd:anySimpleType
//    restricted by xsd:string
//      restricted by xsd:normalizedString
//        restricted by xsd:token
//          restricted by xsd:Name
//            restricted by xsd:NCName
//              restricted by xsd:IDREF
//                used in list xsd:IDREFS

use crate::model::simple_types::idref::IdRef;
use crate::model::simple_types::xsd_list::XsdList;

pub type IdRefs = XsdList<IdRef>;
