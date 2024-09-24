use crate::llm_agent::{LLMAgentOpenAI, LLMAgentHuggingFace};
use log::info;

pub struct IterationOfThought {
    max_iterations: u32,
    timeout: f32,
}

impl IterationOfThought {
    pub fn new(max_iterations: u32, timeout: f32) -> Self {
        Self {
            max_iterations,
            timeout,
        }
    }

    pub async fn aiot(&mut self, query: &str, model_choice: &str) -> String {
        info!("Starting AIoT...");

        let mut current_response = if model_choice == "llama3" {
            let llama_agent = LLMAgentHuggingFace::new();
            llama_agent.call_llm(query, "Initial Prompt").await
        } else {
            let openai_agent = LLMAgentOpenAI::new();
            openai_agent.call_llm(query, "Initial Prompt").await
        };

        for iteration in 1..=self.max_iterations {
            info!("Iteration {}: {}", iteration, current_response);

            if self.stopping_criterion(&current_response) {
                info!("Stopping criterion met.");
                break;
            }

            let new_prompt = self.inner_dialogue_agent(query, &current_response, model_choice).await;
            current_response = if model_choice == "llama3" {
                let llama_agent = LLMAgentHuggingFace::new();
                llama_agent.call_llm(query, &new_prompt).await
            } else {
                let openai_agent = LLMAgentOpenAI::new();
                openai_agent.call_llm(query, &new_prompt).await
            };
        }

        info!("AIoT completed.");
        current_response
    }

    fn stopping_criterion(&self, response: &str) -> bool {
        ["answer:", "final answer:", "conclusion:", "summary:", "the answer is:"]
            .iter()
            .any(|&keyword| response.to_lowercase().contains(keyword))
    }

    async fn inner_dialogue_agent(&self, query: &str, previous_response: &str, model_choice: &str) -> String {
        let prompt = format!(
            "Given the original query: '{}' and the previous response: '{}', generate a new prompt.",
            query, previous_response
        );

        if model_choice == "llama3" {
            let llama_agent = LLMAgentHuggingFace::new();
            llama_agent.call_llm(query, &prompt).await
        } else {
            let openai_agent = LLMAgentOpenAI::new();
            openai_agent.call_llm(query, &prompt).await
        }
    }
}
