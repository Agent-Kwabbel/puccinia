#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Engine {
    Google,
    DuckDuckGo,
    ChatGPT,
    WolframAlpha,
    Fast,
    Local,
}

pub fn parse_engine(input: &str) -> Result<Engine, String> {
    match input.to_lowercase().as_str() {
        "google" | "g" => Ok(Engine::Google),
        "firefox" | "ff" | "fx" | "f" | "duckduckgo" | "ddg" | "duck" | "d" => Ok(Engine::DuckDuckGo),
        "chatgpt" | "cg" | "chat" | "cgpt" | "gpt" | "c" => Ok(Engine::ChatGPT),
        "wolframalpha" | "wolfram" | "alpha" | "wa" => Ok(Engine::WolframAlpha),
        "fast" | "math" | "quick" => Ok(Engine::Fast),
        "local" | "l" | "llm" | "ollama" | "mistral" => Ok(Engine::Local),
        _ => Err(format!("Unknown engine alias: {}", input)),
    }
}

