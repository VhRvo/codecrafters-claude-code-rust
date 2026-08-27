pub fn get_tools() -> Vec<serde_json::Value> {
    vec![get_read_tool()]
}

pub fn get_read_tool() -> serde_json::Value {
    serde_json::from_str(include_str!("tools/read.json")).unwrap()
}

pub fn execute_read(arguments: &serde_json::Map<String, serde_json::Value>) -> Result<String, Box<dyn std::error::Error>> {
    let dir_path = "";
    let file_path = arguments.get("file_path").ok_or("no file_path provided")?.as_str().ok_or("file_path is not a string")?;
    let full_file_path = std::path::Path::new(&dir_path).join(file_path);
    // println!("Reading file: {:?}", full_file_path.as_path());
    let content = std::fs::read_to_string(full_file_path.as_path())?;
    Ok(content)
    // todo!()
}
