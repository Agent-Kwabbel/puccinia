use crate::cli::Cli;
use std::env;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

const CHATGPT_PROMPT_TEMPLATE: &str = r#"
You are a concise, intelligent assistant. Always provide direct, helpful answers without unnecessary explanations unless explicitly asked. Avoid repeating the question.

Q: {user_input}
"#;

#[derive(Debug, Clone)]
pub enum ChatGPTModel {
    O3Mini,
    O1,
    O1Mini,
    FourO,
    FourOMini,
}

impl std::str::FromStr for ChatGPTModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "o3" | "o3m" | "o3-mini" => Ok(Self::O3Mini),
            "o1" => Ok(Self::O1),
            "o1m" | "o1-mini" => Ok(Self::O1Mini),
            "4o" | "four-o" => Ok(Self::FourO),
            "4om" | "4o-mini" | "four-o-mini" => Ok(Self::FourOMini),
            _ => Err(format!("Invalid model: {}", s)),
        }
    }
}

#[derive(Serialize)]
struct ChatGPTRequest {
    model: String,
    messages: Vec<ChatGPTMessage>,
}

#[derive(Serialize)]
struct ChatGPTMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatGPTResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatGPTMessageContent,
}

#[derive(Debug, Deserialize)]
struct ChatGPTMessageContent {
    content: String,
}

pub fn handle(args: &Cli) {
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Missing OPENAI_API_KEY env var");
            return;
        }
    };

    let api_url = match env::var("OPENAI_API_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Missing OPENAI_API_URL env var");
            return;
        }
    };

    let model = format_model(args.model.as_ref().unwrap_or(&ChatGPTModel::FourOMini));

    let prompt = CHATGPT_PROMPT_TEMPLATE.replace("{user_input}", &args.search);

    let user_message = ChatGPTMessage {
        role: "user".to_string(),
        content: prompt,
    };

    println!("Querying ChatGPT [{}]...", model);

    let request_body = ChatGPTRequest {
        model,
        messages: vec![user_message],
    };

    let client = Client::new();
    let response = client
        .post(&api_url)
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send();

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<ChatGPTResponse>() {
                    Ok(chat_response) => {
                        if let Some(choice) = chat_response.choices.first() {
                            println!("\n{}", choice.message.content.trim());
                        } else {
                            eprintln!("No response from ChatGPT.");
                        }
                    }
                    Err(e) => eprintln!("Failed to parse ChatGPT response: {}", e),
                }
            } else {
                eprintln!("ChatGPT API error: {}", resp.status());
                if let Ok(text) = resp.text() {
                    eprintln!("{}", text);
                }
            }
        }
        Err(e) => eprintln!("HTTP request failed: {}", e),
    }
}

fn format_model(model: &ChatGPTModel) -> String {
    match model {
        ChatGPTModel::O3Mini => "gpt-3.5-turbo".to_string(),     // Adjust as needed
        ChatGPTModel::O1 => "gpt-3.5-turbo-0125".to_string(),
        ChatGPTModel::O1Mini => "gpt-3.5-turbo-instruct".to_string(),
        ChatGPTModel::FourO => "gpt-4o".to_string(),
        ChatGPTModel::FourOMini => "gpt-4o-mini".to_string(),    // If applicable
    }
}

