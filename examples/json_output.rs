use turbine_llm::{LLMRequest, Message, OutputFormat, Provider, TurbineClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JSON Example ===");
    let client = TurbineClient::new(Provider::Gemini)?;

    let request = LLMRequest::new("gemini-2.0-flash-exp")
        .with_messages(vec![
            Message::user("Hello! My name is Alice."),
            Message::assistant("Hello Alice! Nice to meet you."),
            Message::user("What's my name?"),
        ])
        .with_system_prompt("Response in Json: {name: value}")
        .with_max_tokens(50)
        .with_output_format(OutputFormat::Json);

    let response = client.send_request(&request).await?;
    println!("Response: {}", response.content);
    println!(
        "Tokens: {} input, {} output\n",
        response.usage.input_tokens, response.usage.output_tokens
    );

    Ok(())
}
