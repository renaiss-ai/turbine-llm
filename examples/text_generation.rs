use turbine_llm::{LLMRequest, Message, Provider, TurbineClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Text Generation Example ===\n");

    // Create a client for OpenAI
    let client = TurbineClient::new(Provider::OpenAI)?;

    // Simple question
    let request = LLMRequest::new("gpt-4o-mini")
        .with_system_prompt("You are a helpful assistant.")
        .with_message(Message::user(
            "Explain what Rust programming language is in 2-3 sentences.",
        ))
        .with_max_tokens(150);

    println!("Sending request to OpenAI...\n");
    let response = client.send_request(&request).await?;

    println!("Response:\n{}\n", response.content);
    println!(
        "Token usage: {} input, {} output",
        response.usage.input_tokens, response.usage.output_tokens
    );

    Ok(())
}
