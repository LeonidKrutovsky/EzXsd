use crate::model::elements::ElementType;
use crate::model::{Binding, Definitions, PortType, Service};
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
            match ch.wsdl_type()? {
                ElementType::Import => res.content.add_import(Import::parse(ch)?)?,
                ElementType::Types => res.content.add_types(Types::parse(ch)?)?,
                ElementType::Message => res.content.add_message(Message::parse(ch)?)?,
                ElementType::PortType => res.content.add_port_type(PortType::parse(ch)?)?,
                ElementType::Binding => res.content.add_binding(Binding::parse(ch)?)?,
                ElementType::Service => res.content.add_service(Service::parse(ch)?)?,
                x => return Err(format!("Invalid child element: {:?}", x)),
            }
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use roxmltree::Document;
    use xsd10::model::simple_types::QName;

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

        assert_eq!(
            def.target_namespace.unwrap().0,
            "http://www.onvif.org/ver10/device/wsdl"
        );
        assert_eq!(def.content.types.first().unwrap().elements.len(), 1);
        let messages = &def.content.messages;

        assert_eq!(messages.len(), 4);
        assert_eq!(
            messages.get("GetServicesRequest").unwrap().part.name.0,
            "parameters"
        );
        assert_eq!(
            messages
                .get("DeleteGeoLocationResponse")
                .unwrap()
                .part
                .element
                .as_ref()
                .unwrap(),
            &QName::from("tds:DeleteGeoLocationResponse")
        );

        if let Some(pt) = def.content.port_types.get("Device") {
            assert_eq!(pt.operations.len(), 2);
            assert_eq!(pt.operations[0].name.0, "GetServices");
            assert!(pt.operations[0].documentation.is_some());
        } else {
            panic!("Test failed! Invalid PortType parsing:  {:?}", def.content);
        }

        if let Some(b) = def.content.bindings.get("DeviceBinding") {
            assert_eq!(b.operations.len(), 2);

            assert_eq!(b.operations[0].elements.len(), 1);
            assert_eq!(b.operations[0].name.0, "GetServices");
            assert!(b.operations[0].documentation.is_none());
        } else {
            panic!("Test failed! Invalid Binding parsing:  {:?}", def.content);
        }
    }
}
