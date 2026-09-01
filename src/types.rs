use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    Tool,
}

pub mod response {
    use super::Role;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Response {
        pub choices: Vec<Choice>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Choice {
        pub message: Message,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Message {
        pub role: Role,
        pub content: Option<String>,
        pub tool_calls: Option<Vec<ToolCall>>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub struct ToolCall {
        pub id: String,
        #[serde(rename = "type")]
        pub call_type: String,
        #[serde(rename = "function")]
        pub function_call: FunctionCall,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FunctionCall {
        pub name: String,
        pub arguments: String,
    }
}

pub mod request {
    use super::Role;
    use serde::Serialize;

    #[derive(Clone, Debug, Serialize)]
    pub struct Request {
        pub messages: Vec<serde_json::Value>,
        pub model: String,
        pub tools: Vec<serde_json::Value>,
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Message {
        pub role: Role,
        pub tool_call_id: String,
        pub content: String,
    }
}

#[cfg(test)]
mod tests {
    use super::{ReadArguments, response::FunctionCall};
    use serde_json::json;

    #[test]
    fn deserializes_read_function_call() {
        let function_call: FunctionCall = serde_json::from_value(json!({
            "name": "Read",
            "arguments": r#"{"file_path":"apple.py"}"#,
        }))
        .expect("function call should deserialize");

        assert_eq!(function_call.name, "Read");

        let arguments: ReadArguments = serde_json::from_str(&function_call.arguments)
            .expect("Read arguments should deserialize");

        assert_eq!(arguments.file_path, "apple.py");
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReadArguments {
    pub file_path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WriteArguments {
    pub file_path: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BashArguments {
    pub command: String,
}
