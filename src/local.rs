use crate::cli::Cli;
use std::process::{Command, Stdio};
use std::io::Write;

const PROMPT_TEMPLATE: &str = r#"
You are a fast and accurate assistant. Answer each question with the final result only. No explanations. Keep answers as short and factual as possible.

Q: 5x = 2 + 3
A: x = 1

Q: Timezone Amsterdam
A: CET (UTC+1)

Q: 3 * (2 + 4) - 1
A: 17

Q: What’s the capital of Japan?
A: Tokyo

Q: Define inertia
A: Resistance to change in motion

Q: {user_input}
A:"#;

pub fn handle(args: &Cli) {
    let user_input = args.search.trim();
    let prompt = PROMPT_TEMPLATE.replace("{user_input}", user_input);

    let mut child = Command::new("ollama")
        .args(&["run", "mistral"]) 
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start local LLM");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let prompt_clone = prompt.clone();

    std::thread::spawn(move || {
        stdin
            .write_all(prompt_clone.as_bytes())
            .expect("Failed to write prompt to stdin");
    });

    let output = child
        .wait_with_output()
        .expect("Failed to read model output");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

