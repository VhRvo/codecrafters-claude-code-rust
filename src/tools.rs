use crate::types::response;

pub fn get_tools() -> Vec<serde_json::Value> {
    vec![get_read_tool(), get_write_tool()]
}

fn get_read_tool() -> serde_json::Value {
    serde_json::from_str(include_str!("tools/read.json")).unwrap()
}

fn get_write_tool() -> serde_json::Value {
    serde_json::from_str(include_str!("tools/write.json")).unwrap()
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

pub fn execute_write(
    arguments: &response::WriteArguments,
) -> Result<String, Box<dyn std::error::Error>> {
    let dir_path = "";
    let file_path = &arguments.file_path;
    let file_path = std::path::Path::new(&dir_path).join(file_path);
    // println!("Writing file: {:?}", full_file_path.as_path());
    std::fs::write(file_path, &arguments.content)?;
    Ok(String::new())
}
