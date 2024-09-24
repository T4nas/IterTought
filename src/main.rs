mod iteration_of_thought;
mod llm_agent;
mod io;

use crate::iteration_of_thought::IterationOfThought;
use crate::io::get_user_query;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();

    let query = get_user_query();
    let mut iot = IterationOfThought::new(5, 30.0);

    // "llama3" via HuggingFace API or "openai" ChatGPT via OpenAI API
    // Declare ENV variable for the API keys
    let model_choice = "llama3";

    let result = iot.aiot(&query, model_choice).await;
    println!("AIoT Result: {}", result);
}
