pub mod binding;
pub mod definitions;
pub mod documentation;
pub mod fault;
pub mod import;
pub mod input;
pub mod message;
pub mod operation;
pub mod output;
pub mod part;
pub mod port;
pub mod port_type;
pub mod service;
pub mod types;

#[derive(Debug, PartialEq)]
pub enum ElementType {
    Binding,
    Definitions,
    Documentation,
    Fault,
    Import,
    Input,
    Message,
    Operation,
    Output,
    Part,
    Port,
    PortType,
    Service,
    Types,

    Unknown(String),
}

pub fn wsdl_element_type(name: &str) -> Result<ElementType, String> {
    use ElementType::*;
    let element = match name {
        "binding" => Binding,
        "definitions" => Definitions,
        "documentation" => Documentation,
        "fault" => Fault,
        "import" => Import,
        "input" => Input,
        "message" => Message,
        "operation" => Operation,
        "output" => Output,
        "part" => Part,
        "port" => Port,
        "portType" => PortType,
        "service" => Service,
        "types" => Types,
        _ => return Err(format!("Invalid wsdl element name: {}", name)),
    };
    Ok(element)
}
