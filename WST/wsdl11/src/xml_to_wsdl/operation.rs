use crate::model::complex_types::t_operation::OperationContent;
use crate::model::elements::ElementType;
use crate::model::{
    BindingOperation, BindingOperationFault, BindingOperationInput, BindingOperationOutput,
    Documentation, Fault, Input, Operation, Output,
};
use crate::xml_to_wsdl::documentation::documentation_first;
use crate::xml_to_wsdl::WsdlNode;
use roxmltree::Node;
use xsd10::model::simple_types::NCName;
use xsd10::xml_to_xsd::ElementChildren;

impl<'a> Operation<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        let mut name = None;

        for attr in node.attributes() {
            match attr.name() {
                "name" => name = Some(NCName::from(attr)),
                "parameterOrder" => res.parameter_order = Some(attr.value()),
                _ => return Err(format!("Invalid attribute. {:?}", node)),
            }
        }

        res.name = name.ok_or_else(|| format!("Name attribute required: {:?}", node))?;
        res.documentation = documentation_first(node)?;
        res.content = OperationContent::parse(node)?;

        Ok(res)
    }
}

impl<'a> OperationContent<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut children = node.children().filter(|n| n.is_element());

        while let Some(ch) = children.next() {
            match ch.wsdl_type()? {
                ElementType::Input => {
                    // wsdl:request-response-or-one-way-operation
                    return Ok(if let Some(output_node) = children.next() {
                        // RequestResponse
                        OperationContent::RequestResponse {
                            input: Input::parse(ch)?,
                            output: Output::parse(output_node)?,
                            faults: children
                                .filter_map(|n| match n.wsdl_type() {
                                    Ok(ElementType::Fault) => Some(Fault::parse(n)),
                                    _ => None,
                                })
                                .collect::<Result<Vec<_>, String>>()?,
                        }
                    } else {
                        // OneWay
                        OperationContent::OneWay {
                            input: Input::parse(ch)?,
                        }
                    });
                }
                ElementType::Output => {
                    // wsdl:solicit-response-or-notification-operation
                    return Ok(if let Some(input_node) = children.next() {
                        // SolicitResponse
                        assert_eq!(input_node.wsdl_type()?, ElementType::Input);
                        OperationContent::SolicitResponse {
                            output: Output::parse(ch)?,
                            input: Input::parse(input_node)?,
                            faults: children
                                .filter_map(|n| match n.wsdl_type() {
                                    Ok(ElementType::Fault) => Some(Fault::parse(n)),
                                    _ => None,
                                })
                                .collect::<Result<Vec<_>, String>>()?,
                        }
                    } else {
                        OperationContent::Notification {
                            output: Output::parse(ch)?,
                        }
                    });
                }
                _ => continue,
            };
        }
        unreachable!("Content of wsdl:operation must contain input or output")
    }
}

impl<'a> BindingOperation<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        let mut name = None;

        for attr in node.attributes() {
            match attr.name() {
                "name" => name = Some(NCName::from(attr)),
                _ => return Err(format!("Invalid attribute. {:?}", node)),
            }
        }

        res.name = name.ok_or_else(|| format!("Name attribute required: {:?}", node))?;

        for ch in node.element_children() {
            match ch.wsdl_type() {
                Ok(ElementType::Documentation) => {
                    res.documentation = Some(Documentation::parse(ch)?)
                }
                Ok(ElementType::Input) => res.input = Some(BindingOperationInput::parse(ch)?),
                Ok(ElementType::Output) => res.input = Some(BindingOperationOutput::parse(ch)?),
                Ok(ElementType::Fault) => res.faults.push(BindingOperationFault::parse(ch)?),
                _ => res.elements.push(ch),
            }
        }

        Ok(res)
    }
}
