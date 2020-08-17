// wsdl:tOperation
// Complex type information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:documentation [0..1]   from type wsdl:tDocumented
//     Any element [0..*] Namespace: ##other, Process Contents: lax    from type wsdl:tExtensibleDocumented
//     Choice [1..1]
//         Sequence [1..1]     from group wsdl:request-response-or-one-way-operation
//             wsdl:input [1..1]
//             Sequence [0..1]
//                 wsdl:output [1..1]
//                 wsdl:fault [0..*]
//
//         Sequence [1..1]     from group wsdl:solicit-response-or-notification-operation
//             wsdl:output [1..1]
//             Sequence [0..1]
//                 wsdl:input [1..1]
//                 wsdl:fault [0..*]
// Attributes
//     name	        [1..1]	xsd:NCName
//     parameterOrder	[0..1]	xsd:NMTOKENS
// Used by
//     Element wsdl:operation
// Type inheritance chain
//     wsdl:tDocumented
//         wsdl:tExtensibleDocumented
//             wsdl:tOperation

use crate::model::complex_types::t_documentation::Documentation;
use crate::model::elements::fault::Fault;
use crate::model::elements::input::Input;
use crate::model::elements::output::Output;
use crate::model::RawElement;
use xsd10::model::simple_types as xsd;

#[derive(Default, Debug)]
pub struct Operation<'a> {
    pub documentation: Option<Documentation<'a>>,
    pub elements: Vec<RawElement<'a>>,
    pub content: OperationContent<'a>,
    pub name: xsd::NCName<'a>,
    pub parameter_order: Option<&'a str>, //TODO: xsd::NMTOKENS<'a>,
}

// Choice [1..1]
//     Sequence [1..1]     from group wsdl:request-response-or-one-way-operation
//         wsdl:input [1..1]
//         Sequence [0..1]
//             wsdl:output [1..1]
//             wsdl:fault [0..*]
//     Sequence [1..1]     from group wsdl:solicit-response-or-notification-operation
//         wsdl:output [1..1]
//         Sequence [0..1]
//             wsdl:input [1..1]
//             wsdl:fault [0..*]
// Choice between RequestResponseOrOneWayOperation and SolicitResponseOrNotificationOperation
#[derive(Debug)]
pub enum OperationContent<'a> {
    RequestResponse {
        input: Input<'a>,
        output: Output<'a>,
        faults: Vec<Fault<'a>>,
    },
    OneWay {
        input: Input<'a>,
    },
    SolicitResponse {
        output: Output<'a>,
        input: Input<'a>,
        faults: Vec<Fault<'a>>,
    },
    Notification {
        output: Output<'a>,
    },
}

impl Default for OperationContent<'_> {
    fn default() -> Self {
        Self::OneWay {
            input: Input::default(),
        }
    }
}
