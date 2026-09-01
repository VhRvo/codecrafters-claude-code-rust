use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, json};
use std::{env, process};

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

    #[allow(unused_variables)]
    let response: Value = client
        .chat()
        .create_byot(json!({
            "messages": [
                {
                    "role": "user",
                    "content": args.prompt
                }
            ],
            "model": model,
            "tools": tools
        }))
        .await?;

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");
    // eprintln!("{}", to_string_pretty(&response)?);

    let response: types::response::Response = serde_json::from_value(response.clone())?;
    let message = &response.choices[0].message;

    if let Some(tool_calls) = &message.tool_calls {
        let tool_call = &tool_calls[0];
        let function_call = &tool_call.function_call;
        let name = &function_call.name;
        let arguments = &function_call.arguments;
        let arguments: types::response::ReadArguments = serde_json::from_str(arguments)?;
        if name != "Read" {
            return Err(format!("unknown tool: {}", name).into());
        }
        let result = tools::execute_read(&arguments)?;
        println!("{}", result);
    } else if let Some(content) = &message.content {
        println!("{}", content);
    }

    Ok(())
}
