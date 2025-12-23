use turbine_llm::TurbineClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simplified Usage Example ===\n");

    // Create client with model string - automatically detects provider
    // Supported formats:
    // - "openai/gpt-4o-mini" or just "gpt-4o-mini"
    // - "google/gemini-flash" or just "gemini-flash"
    // - "anthropic/claude-3-5-sonnet" or just "claude-3-5-sonnet"
    // - "groq/llama-3.1-8b" or "llama-3.1-8b" or "mixtral-8x7b"

    println!("Creating client for OpenAI...");
    let client = TurbineClient::from_model("openai/gpt-4o-mini")?;

    // Simple one-liner to send a message
    println!("\nSending message...\n");
    let response = client.send("Explain Rust in one sentence.").await?;

    println!("Response:\n{}\n", response.content);
    println!(
        "Token usage: {} input, {} output\n",
        response.usage.input_tokens, response.usage.output_tokens
    );

    // Can also use with system prompt
    println!("Sending message with system prompt...\n");
    let response = client
        .send_with_system("You are a concise technical writer", "What is async/await?")
        .await?;

    println!("Response:\n{}\n", response.content);
    println!(
        "Token usage: {} input, {} output",
        response.usage.input_tokens, response.usage.output_tokens
    );

    Ok(())
}
