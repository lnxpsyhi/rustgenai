pub mod genai {
    use core::f64;

    use clap::Parser;
    use reqwest::Client;
    use serde_json::{Value, json};

    #[derive(Debug, Parser)]
    #[command(version, about, long_about = None)]
    pub struct GoogleGenAi {
        #[arg(short, long)]
        api_key: String,
        #[arg(short, long)]
        contents: String,
    }

    impl GoogleGenAi {
        pub async fn generate_content(&self) -> Result<(), reqwest::Error> {
            let url = format!(
                "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
                self.api_key
            );

            let client = Client::new();

            let request_body = json!({
                "contents": [
                    {
                        "parts": [
                            {
                                "text": self.contents
                            }
                        ]
                    }
                ]
            });

            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .body(request_body.to_string())
                .send()
                .await?;

            if response.status().is_success() {
                let response_text = response.text().await?;
                let parsed: Value = serde_json::from_str(&response_text).unwrap();
                let candidates = parsed["candidates"].as_array().unwrap();
                let best_candidate = candidates.iter().max_by(|a, b| {
                    let logprob_a = a["avgLogprobs"].as_f64().unwrap_or(f64::NEG_INFINITY);
                    let logprob_b = b["avgLogprobs"].as_f64().unwrap_or(f64::NEG_INFINITY);
                    logprob_a
                        .partial_cmp(&logprob_b)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                if let Some(candidate) = best_candidate {
                    if let Some(text) = candidate["content"]["parts"][0]["text"].as_str() {
                        println!("Best Candidate: {}", text);
                    } else {
                        println!("No text found for the best candidate.");
                    }
                }
            } else {
                eprintln!("Request failed with status: {}", response.status());
            }

            Ok(())
        }
    }
}
