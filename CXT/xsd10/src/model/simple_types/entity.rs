// xsd:ENTITY
// The type xsd:ENTITY represents a reference to an unparsed entity.
// The xsd:ENTITY type is most often used to include information from another
// location that is not in XML format, such as graphics. An xsd:ENTITY value must
// be an NCName. An xsd:ENTITY value carries the additional constraint that it
// must match the name of an unparsed entity in a document type definition (DTD) for the instance.
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
// Examples
// The example shows an XML document that links product numbers to pictures of the products. In the schema, the picture element declaration declares an attribute location that has the type xsd:ENTITY. In the instance, each value of the location attribute (in this case, prod557 and prod563) matches the name of an entity declared in the internal DTD subset of the instance.
//
// Schema:
//
// <xs:element name="picture">
//   <xs:complexType>
//     <xs:attribute name="location" type="xs:ENTITY"/>
//   </xs:complexType>
// </xs:element>
// <!--...-->
// Instance:
//
// <!DOCTYPE catalog SYSTEM "catalog.dtd" [
// <!NOTATION jpeg SYSTEM "JPG">
// <!ENTITY prod557 SYSTEM "prod557.jpg" NDATA jpeg>
// <!ENTITY prod563 SYSTEM "prod563.jpg" NDATA jpeg>
// ]>
//
// <catalog>
//   <product>
//     <number>557</number>
//     <picture location="prod557"/>
//   </product>
//   <product>
//     <number>563</number>
//     <picture location="prod563"/>
//   </product>
// </catalog>

// Type Inheritance Chain
//  xsd:anySimpleType
//    restricted by xsd:string
//      restricted by xsd:normalizedString
//        restricted by xsd:token
//          restricted by xsd:Name
//            restricted by xsd:NCName
//              restricted by xsd:ENTITY
//                used in list xsd:ENTITIES

use crate::model::simple_types::NCName_;

pub type Entity<'a> = NCName_<'a>;
