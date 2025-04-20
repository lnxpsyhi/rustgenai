// extern crate dotenv;

// use dotenv::dotenv;
// use std::env;

use clap::Parser;
use google::genai::GoogleGenAi;

mod google;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // dotenv().ok();
    // let api_key = env::var("GEMINI_API_KEY").unwrap();

    let genai = GoogleGenAi::parse();

    genai.generate_content().await?;

    Ok(())
}
