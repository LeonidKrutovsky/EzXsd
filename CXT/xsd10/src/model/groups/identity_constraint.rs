use crate::model::elements::key::Key;
use crate::model::elements::key_ref::KeyRef;
use crate::model::elements::unique::Unique;

// xsd:identityConstraint
// The three kinds of identity constraints, all with type of or derived from 'keybase'.
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      xsd:unique
//      xsd:key
//      xsd:keyref
//
// Used in
// Group xsd:elementModel
#[derive(Debug)]
pub enum IdentityConstraint {
    Unique(Unique),
    Key(Key),
    KeyRef(KeyRef),
}
