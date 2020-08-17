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
use crate::model::elements::input::ParamInput;
use crate::model::elements::output::ParamOutput;

#[derive(Debug)]
pub enum SolicitResponseOrNotificationOperation<'a> {
    SolicitResponse {
        output: ParamOutput<'a>,
        input: ParamInput<'a>,
        faults: Vec<Fault<'a>>,
    },
    Notification {
        output: ParamOutput<'a>,
    },
}
