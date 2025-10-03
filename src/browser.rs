use std::process::Command;
use crate::cli::Cli;
use crate::engine::Engine;

pub fn handle(_engine: &Engine, args: &Cli) {
    let search_url = match _engine {
        Engine::Google => format!("https://www.google.com/search?q={}", urlencoding::encode(&args.search)),
        Engine::DuckDuckGo => format!("https://duckduckgo.com/?q={}", urlencoding::encode(&args.search)),
        _ => panic!("Unsupported engine in browser handler"),
    };

    let browser_flag = if args.private {
        "--private-window"
    } else if args.new_tab {
        "--new-tab"
    } else {
        "--new-tab" // default behavior
    };

    println!("→ Launching zen-browser with search: {}", args.search);
    let status = Command::new("zen-browser")
        .args(&[browser_flag, &search_url])
        .status()
        .expect("Failed to launch zen-browser");

    if !status.success() {
        eprintln!("zen-browser exited with status: {}", status);
    }
}

