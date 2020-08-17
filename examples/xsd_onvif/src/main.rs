use roxmltree::Document;
use std::fs;
use std::io::Read;
use std::path::Path;
use xsd10::xml_to_xsd::schema_set::SchemaSet;

fn main() {
    let input_path = Path::new("../../CXT/xsd10/input/xsd");
    let sources = load_files(&input_path).unwrap();
    let xml_docs = parse_xml_files(sources.as_slice()).unwrap();
    let schema_set = SchemaSet::from_docs(xml_docs.as_slice()).unwrap();

    for wrapper in schema_set.schemas() {
        println!("{:#?}", wrapper.schema());
    }
}

fn load_files(path: &Path) -> Result<Vec<String>, String> {
    let md = fs::metadata(path).map_err(|err| err.to_string())?;
    let mut res = vec![];
    if md.is_dir() {
        res = load_dir(path)?;
    } else if md.is_file() {
        res.push(load_file(&path)?);
    } else {
        panic!("symlink path")
    }

    Ok(res)
}

fn load_dir(input_path: &Path) -> Result<Vec<String>, String> {
    let mut res = vec![];
    for entry in fs::read_dir(input_path).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.is_dir() {
            res.append(&mut load_dir(&path)?);
        } else {
            res.push(load_file(&path)?)
        }
    }
    Ok(res)
}

fn load_file(path: &Path) -> Result<String, String> {
    let mut file = fs::File::open(&path).map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;
    Ok(text)
}

fn parse_xml_files(files: &[String]) -> Result<Vec<Document>, String> {
    let s: Result<Vec<_>, _> = files.iter().map(|f| Document::parse(f.as_str())).collect();
    s.map_err(|err| err.to_string())
}
