use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, json, to_string_pretty};
use std::{env, process};

mod tools;

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

    let choice = &response["choices"][0];
    // eprintln!("{}", to_string_pretty(&choice)?);
    let message = &choice["message"];
    // eprintln!("{}", to_string_pretty(&message)?);
    let tool_calls = &message["tool_calls"];
    // eprintln!("{}", to_string_pretty(&tool_calls)?);

    if tool_calls.is_null() {
        let content = message["content"].as_str().ok_or("no message provided")?;
        println!("{}", content);
    } else if let Some(tool_call) = tool_calls.as_array().ok_or("tool_calls is not an array")?.get(0) {
        let function = tool_call["function"].as_object().ok_or("function is not an object")?;
        // println!("function: {:?}", function);
        let name = function.get("name").ok_or("no name provided")?.as_str().ok_or("name is not a string")?;
        let arguments = function["arguments"].as_str().ok_or("arguments is not an path")?;
        let arguments = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(arguments)?;
        if name != "Read" {
            return Err(format!("unknown tool: {}", name).into());
        }
        let result = tools::execute_read(&arguments)?;
        println!("{}", result);
    }

    Ok(())
}
