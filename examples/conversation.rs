use turbine_llm::{LLMRequest, Message, Provider, TurbineClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Multi-turn Conversation Example ===\n");

    // Create a client for Gemini
    let client = TurbineClient::new(Provider::Gemini)?;

    // Build a conversation with multiple turns
    let messages = vec![
        Message::user("Hello! My name is Alice and I love programming."),
        Message::assistant(
            "Hello Alice! It's great to meet you. Programming is wonderful! What languages do you enjoy?",
        ),
        Message::user("I really like Rust! What's my name again?"),
    ];

    let request = LLMRequest::new("gemini-2.0-flash-exp")
        .with_system_prompt("You are a friendly assistant with a good memory.")
        .with_messages(messages)
        .with_max_tokens(100);

    println!("Sending conversation to Gemini...\n");
    let response = client.send_request(&request).await?;

    println!("Response:\n{}\n", response.content);
    println!(
        "Token usage: {} input, {} output",
        response.usage.input_tokens, response.usage.output_tokens
    );

    Ok(())
}
