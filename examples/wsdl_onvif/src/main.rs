use roxmltree::Document;
use std::fs;
use std::io::Read;
use std::path::Path;
use wsdl11::model::Definitions;
use xsd10::xml_to_xsd::schema_set::SchemaSet;

fn main() {
    let input_path = Path::new("../../WST/wsdl11/input/wsdl/deviceio.wsdl.xml");
    if let Err(e) = process_single_file(&input_path) {
        println!("{}", e);
    }
}

fn process_single_file(input_path: &Path) -> Result<(), String> {
    let text = load_file(input_path)?;
    let doc = Document::parse(text.as_str()).unwrap();
    let definitions = Definitions::parse(doc.root_element())?;
    println!("{:#?}", definitions);

    let mut schema_set = SchemaSet::default();

    definitions.content.types.iter().for_each(|t| {
        t.elements.iter().for_each(|e| {
            if let Err(e) = schema_set.add_schema(e.clone()) {
                panic!("Error: {}", e);
            }
        })
    });

    for wrapper in schema_set.schemas() {
        println!("{:#?}", wrapper.schema());
    }

    Ok(())
}

fn load_file(path: &Path) -> Result<String, String> {
    let mut file = fs::File::open(&path).map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;
    Ok(text)
}
