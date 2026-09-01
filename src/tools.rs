use crate::types::response;

pub fn get_tools() -> Vec<serde_json::Value> {
    vec![get_read_tool()]
}

pub fn get_read_tool() -> serde_json::Value {
    serde_json::from_str(include_str!("tools/read.json")).unwrap()
}

pub fn execute_read(
    arguments: &response::ReadArguments,
) -> Result<String, Box<dyn std::error::Error>> {
    let dir_path = "";
    let file_path = &arguments.file_path;
    let file_path = std::path::Path::new(&dir_path).join(file_path);
    // println!("Reading file: {:?}", full_file_path.as_path());
    let content = std::fs::read_to_string(file_path)?;
    Ok(content)
}
