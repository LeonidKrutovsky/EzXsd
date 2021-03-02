// xsd:ENTITIES
// The type xsd:ENTITIES represents a list of ENTITY values separated by whitespace.
// There must be at least one ENTITY in the list. Each of the ENTITY values must match
// the name of an unparsed entity that has been declared in a document type definition
// (DTD) for the instance.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:ENTITY
// Minimum Length: 1
// White Space: collapse
// Examples
// The example shows the declaration of an attribute named location that is of type xsd:ENTITIES. In the instance, the location attribute can include a list of entity names. Each value (in this case there are two: prod557a and prod557b) matches the name of an entity that is declared in the internal DTD subset for the instance.
//
// Schema:
//
// <xs:element name="pictures">
//   <xs:complexType>
//     <xs:attribute name="location" type="xs:ENTITIES"/>
//   </xs:complexType>
// </xs:element>
// Instance:
//
// <!DOCTYPE catalog SYSTEM "catalog.dtd" [
// <!NOTATION jpeg SYSTEM "JPG">
// <!ENTITY prod557a SYSTEM "prod557a.jpg" NDATA jpeg>
// <!ENTITY prod557b SYSTEM "prod557b.jpg" NDATA jpeg>
// ]>
//
// <catalog>
//   <product>
//     <number>557</number>
//     <pictures location="prod557a prod557b"/>
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

use crate::model::simple_types::entity::Entity;
use crate::model::simple_types::xsd_list::XsdList;

pub type Entities = XsdList<Entity>;
