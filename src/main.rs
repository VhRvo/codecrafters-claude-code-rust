use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde::Serialize;
use serde_json::{Value, json};
use std::{env, process};

use crate::types::request;

mod tools;
mod types;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let model =
        env::var("OPENROUTER_MODEL").unwrap_or_else(|_| "anthropic/claude-haiku-4.5".to_string());

    let tools = tools::get_tools();

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);
    let messages = vec![json!({
        "role": types::Role::User,
        "content": args.prompt.clone(),
    })];
    let mut request = request::Request {
        messages,
        model,
        tools,
    };

    eprintln!("Logs from your program will appear here!");
    'outer: loop {
        // eprintln!("request: \n{}", serde_json::to_string_pretty(&request)?);
        let response: Value = client.chat().create_byot(request.clone()).await?;

        // You can use print statements as follows for debugging, they'll be visible when running tests.
        // eprintln!("response: \n{}", serde_json::to_string_pretty(&response)?);

        let response: types::response::Response = serde_json::from_value(response.clone())?;
        for choice in response.choices {
            let message = choice.message;

            if let Some(tool_calls) = &message.tool_calls {
                request.messages.push(serde_json::to_value(&message)?);
                for tool_call in tool_calls {
                    let result = evaluate_tool_call(tool_call)?;
                    // println!("{}", result);
                    request
                        .messages
                        .push(serde_json::to_value(request::Message {
                            role: types::Role::Tool,
                            tool_call_id: tool_call.id.clone(),
                            content: result,
                        })?);
                }
            } else if let Some(content) = &message.content {
                println!("{}", content);
                break 'outer;
            }
        }
    }

    Ok(())
}

fn evaluate_tool_call(
    tool_call: &types::response::ToolCall,
) -> Result<String, Box<dyn std::error::Error>> {
    let function_call = &tool_call.function_call;
    let name = &function_call.name;
    let arguments = &function_call.arguments;
    let result = match name.as_str() {
        "Read" => {
            let arguments: types::response::ReadArguments = serde_json::from_str(arguments)?;
            tools::execute_read(&arguments)
        }
        "Write" => {
            let arguments: types::response::WriteArguments = serde_json::from_str(arguments)?;
            tools::execute_write(&arguments)
        }
        _ => Err(format!("unknown tool: {}", name).into()),
    }?;
    Ok(result)
}
