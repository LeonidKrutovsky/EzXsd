// wsdl:solicit-response-or-notification-operation
// Group information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:output [1..1]
//     Sequence [0..1]
//         wsdl:input [1..1]
//         wsdl:fault [0..*]
// Used in
//     Type wsdl:tOperation (Element wsdl:operation)

use crate::model::elements::fault::Fault;
use crate::model::elements::input::Input;
use crate::model::elements::output::ParamOutput;

#[derive(Debug)]
pub struct SolicitResponseOrNotificationOperation<'a> {
    output: ParamOutput<'a>,
    content: Option<SolicitResponse<'a>>,
}

#[derive(Debug)]
pub struct SolicitResponse<'a> {
    input: Input<'a>,
    faults: Vec<Fault<'a>>,
}
