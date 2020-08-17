use crate::model::{Operation, Documentation, Output, Input, Fault};
use roxmltree::Node;
use xsd10::model::simple_types::NCName;
use xsd10::xml_to_xsd::ElementChildren;
use crate::xml_to_wsdl::WsdlNode;
use crate::model::elements::ElementType;
use crate::model::complex_types::t_operation::OperationContent;

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

        res.name = name
            .ok_or_else(|| format!("Name attribute required: {:?}", node))?
            .into();

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
                        assert_eq!(
                            output_node.wsdl_type()?,
                            ElementType::Output,
                            "{}",
                            format!("{:?}", output_node)
                        );
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