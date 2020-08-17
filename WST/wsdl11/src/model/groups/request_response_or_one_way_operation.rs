// wsdl:request-response-or-one-way-operation
// Group information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
//
// Content
//  Sequence [1..1]
//     wsdl:input [1..1]
//     Sequence [0..1]
//         wsdl:output [1..1]
//         wsdl:fault [0..*]
// Used in
//     Type wsdl:tOperation (Element wsdl:operation)

#[derive(Clone, Debug, PartialEq)]
pub enum RequestResponseOrOneWayOperation<'a> {
    RequestResponse {
        input: ParamInput<'a>,
        output: ParamOutput<'a>,
        faults: Vec<Fault<'a>>,
    },
    OneWay {
        input: ParamInput<'a>,
    },
}