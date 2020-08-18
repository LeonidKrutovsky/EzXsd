use crate::model::elements::ElementType;
use crate::model::groups::any_top_level_optional_element::AnyTopLevelOptionalElement;
use crate::model::{Binding, Definitions, PortType};
use crate::model::{Import, Message, Types};
use crate::xml_to_wsdl::WsdlNode;
use roxmltree::Node;
use xsd10::model::simple_types::{AnyUri, NCName};
use xsd10::xml_to_xsd::ElementChildren;

impl<'a> Definitions<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for attr in node.attributes() {
            match attr.name() {
                "targetNamespace" => res.target_namespace = Some(AnyUri::from(attr)),
                "name" => res.name = Some(NCName::from(attr)),
                x => return Err(format!("Invalid attribute: {}", x)),
            }
        }

        for ch in node.element_children() {
            res.content.push(parse_content(ch)?);
        }

        res.content = node
            .children()
            .filter(|n| n.is_element())
            .map(parse_content)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(res)
    }
}

fn parse_content<'a>(node: Node<'a, '_>) -> Result<AnyTopLevelOptionalElement<'a>, String> {
    match node.wsdl_type()? {
        ElementType::Import => Ok(AnyTopLevelOptionalElement::Import(Import::parse(node)?)),
        ElementType::Types => Ok(AnyTopLevelOptionalElement::Types(Types::parse(node)?)),
        ElementType::Message => Ok(AnyTopLevelOptionalElement::Message(Message::parse(node)?)),
        ElementType::PortType => Ok(AnyTopLevelOptionalElement::PortType(PortType::parse(node)?)),
        ElementType::Binding => Ok(AnyTopLevelOptionalElement::Binding(Binding::parse(node)?)),
        ElementType::Service => unimplemented!(),
        x => return Err(format!("Invalid child element: {:?}", x)),
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use roxmltree::Document;

    const TEXT: &str = r#"
<wsdl:definitions
    xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/"
    xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap12/"
    xmlns:xs="http://www.w3.org/2001/XMLSchema"
    xmlns:tds="http://www.onvif.org/ver10/device/wsdl"
    targetNamespace="http://www.onvif.org/ver10/device/wsdl">
	<wsdl:types>
		<xs:schema
            targetNamespace="http://www.onvif.org/ver10/device/wsdl"
            xmlns:tt="http://www.onvif.org/ver10/schema"
            xmlns:tds="http://www.onvif.org/ver10/device/wsdl"
            elementFormDefault="qualified" version="18.12"
		>
			<xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../../../ver10/schema/onvif.xsd"/>
		</xs:schema>
	</wsdl:types>

	<wsdl:message name="GetServicesRequest">
		<wsdl:part name="parameters" element="tds:GetServices"/>
	</wsdl:message>
	<wsdl:message name="GetServicesResponse">
		<wsdl:part name="parameters" element="tds:GetServicesResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteGeoLocationRequest">
		<wsdl:part name="parameters" element="tds:DeleteGeoLocation"/>
	</wsdl:message>
	<wsdl:message name="DeleteGeoLocationResponse">
		<wsdl:part name="parameters" element="tds:DeleteGeoLocationResponse"/>
	</wsdl:message>

	<wsdl:portType name="Device">
		<wsdl:operation name="GetServices">
			<wsdl:documentation>Returns information about services on the device.</wsdl:documentation>
			<wsdl:input message="tds:GetServicesRequest"/>
			<wsdl:output message="tds:GetServicesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteGeoLocation">
			<wsdl:documentation>
				This operation deletes the given geo location entries.
			</wsdl:documentation>
			<wsdl:input message="tds:DeleteGeoLocationRequest"/>
			<wsdl:output message="tds:DeleteGeoLocationResponse"/>
		</wsdl:operation>
	</wsdl:portType>

	<wsdl:binding name="DeviceBinding" type="tds:Device">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<wsdl:operation name="GetServices">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetServices"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteGeoLocation">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/DeleteGeoLocation"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
	</wsdl:binding>
</wsdl:definitions>
    "#;

    #[test]
    fn test_definititions() {
        let doc = Document::parse(TEXT).unwrap();
        let def = Definitions::parse(doc.root_element()).unwrap();

        assert_eq!(def.target_namespace.unwrap().0, "http://www.onvif.org/ver10/device/wsdl");
    }


}
