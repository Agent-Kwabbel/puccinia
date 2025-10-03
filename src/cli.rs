use clap::Parser;
use crate::engine::{Engine, parse_engine};
use crate::chatgpt::ChatGPTModel;

#[derive(Parser, Debug)]
#[command(
    name = "puccinia",
    version,
    about = "Run engine-specific search queries from your terminal.",
    long_about = r#"
Supported Engines (use as positional argument or --engine):

  google, g           → Opens Google search in Zen browser
  duckduckgo, ddg, d  → Opens DuckDuckGo in Zen browser
  chatgpt, cg, gpt    → Uses ChatGPT via OpenAI API
  wolfram, wa         → Uses Wolfram Alpha API (simple, short, full)
  fast, math          → Local math evaluator
  local, l, llm       → Local LLM (via Ollama, e.g., phi/tinyllama)

Flags (engine-specific):

  --private, --new-tab           → Google/DuckDuckGo
  --model <MODEL>                → ChatGPT (4o, 4om, o3, o1, o1m)
  --simple, --short, --full      → Wolfram Alpha
"#
)]
pub struct Cli {
    /// Engine name or alias
    pub engine: String,

    /// Search query string
    pub search: String,

    /// Open in a private browser window (Google/DuckDuckGo)
    #[arg(long)]
    pub private: bool,

    /// Open in a new tab (Google/DuckDuckGo)
    #[arg(long)]
    pub new_tab: bool,

    // ChatGPT model (40, 4om, o3, o1, o1m)
    #[arg(long)]
    pub model: Option<ChatGPTModel>,

    // Wolfram Alpha modes
    #[arg(long)]
    pub simple: bool,

    #[arg(long)]
    pub short: bool,

    #[arg(long)]
    pub full: bool,
}

impl Cli {
    pub fn get_engine(&self) -> Engine {
        parse_engine(&self.engine).expect("Unknown engine name or alias")
    }
}

