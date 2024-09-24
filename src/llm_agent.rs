use reqwest::Client;
use serde_json::json;
use log::error;
use dotenv::dotenv;
use std::env;

pub struct LLMAgentOpenAI {
    client: Client,
}

impl LLMAgentOpenAI {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            client: Client::new(),
        }
    }

    pub async fn call_llm(&self, query: &str, prompt: &str) -> String {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

        let request_body = json!({
            "model": "gpt-4",  // Modèle OpenAI à spécifier
            "prompt": prompt,
            "max_tokens": 1000
        });

        let res = self.client
            .post("https://api.openai.com/v1/completions")
            .bearer_auth(api_key)
            .json(&request_body)
            .send()
            .await;

        match res {
            Ok(response) => {
                let text: serde_json::Value = response.json().await.unwrap();
                text["choices"][0]["text"].as_str().unwrap_or("").to_string()
            },
            Err(e) => {
                error!("Failed to call OpenAI API: {}", e);
                String::new()
            }
        }
    }
}

pub struct LLMAgentHuggingFace {
    client: Client,
}

impl LLMAgentHuggingFace {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            client: Client::new(),
        }
    }

    pub async fn call_llm(&self, query: &str, prompt: &str) -> String {
        let token = env::var("HUGGING_FACE_TOKEN").expect("HUGGING_FACE_TOKEN must be set");

        let request_body = json!({
            "inputs": prompt,
        });

        let res = self.client
            .post("https://api-inference.huggingface.co/models/llama-3b") // URL modèle à adapter
            .bearer_auth(token)
            .json(&request_body)
            .send()
            .await;

        match res {
            Ok(response) => {
                let text: serde_json::Value = response.json().await.unwrap();
                text["generated_text"].as_str().unwrap_or("").to_string()
            },
            Err(e) => {
                error!("Failed to call Hugging Face API: {}", e);
                String::new()
            }
        }
    }
}
