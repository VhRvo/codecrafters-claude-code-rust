use crate::types::{BashArguments, ReadArguments, WriteArguments};

pub fn get_tools() -> Vec<serde_json::Value> {
    vec![get_read_tool(), get_write_tool(), get_bash_tool()]
}

fn get_read_tool() -> serde_json::Value {
    serde_json::from_str(include_str!("tools/read.json")).unwrap()
}

fn get_write_tool() -> serde_json::Value {
    serde_json::from_str(include_str!("tools/write.json")).unwrap()
}

fn get_bash_tool() -> serde_json::Value {
    serde_json::from_str(include_str!("tools/bash.json")).unwrap()
}

pub fn execute_read(arguments: &ReadArguments) -> Result<String, Box<dyn std::error::Error>> {
    let dir_path = "";
    let file_path = &arguments.file_path;
    let file_path = std::path::Path::new(&dir_path).join(file_path);
    // println!("Reading file: {:?}", full_file_path.as_path());
    let content = std::fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn execute_write(arguments: &WriteArguments) -> Result<String, Box<dyn std::error::Error>> {
    let dir_path = "";
    let file_path = &arguments.file_path;
    let file_path = std::path::Path::new(&dir_path).join(file_path);
    // println!("Writing file: {:?}", full_file_path.as_path());
    std::fs::write(file_path, &arguments.content)?;
    Ok(String::new())
}

pub fn execute_bash(arguments: &BashArguments) -> Result<String, Box<dyn std::error::Error>> {
    let command = &arguments.command;
    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Command failed with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
