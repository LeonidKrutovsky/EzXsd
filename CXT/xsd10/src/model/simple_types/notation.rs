// xsd:NOTATION
// The type xsd:NOTATION represents a reference to a notation. A notation is a method of interpreting XML and non-XML content. For example, if an element in an XML document contains binary graphics data in JPEG format, a notation can be declared to indicate that this is JPEG data. An attribute of type xsd:NOTATION can then be used to indicate which notation applies to the element's content. A xsd:NOTATION value must be a QName.
//
// xsd:NOTATION is the only built-in type that cannot be the type of attributes or elements. Instead, you must define a new type that restricts xsd:NOTATION, applying one or more enumeration facets. Each of these enumeration values must match the name of a declared notation.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse

// Type Inheritance Chain
//  xsd:anySimpleType
//    restricted by xsd:NOTATION

use crate::model::simple_types::QName;

pub type Notation<'a> = QName<'a>;
