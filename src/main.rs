mod cli;
mod engine;
mod chatgpt;
mod wolfram;
mod browser;
mod fast;
mod local;

use cli::Cli;
use engine::Engine;
use clap::Parser;

fn main() {
    dotenv::dotenv().ok();
    let args = Cli::parse();
    let engine = args.get_engine();

    match engine {
        Engine::Google | Engine::DuckDuckGo => browser::handle(&engine, &args),
        Engine::ChatGPT => chatgpt::handle(&args),
        Engine::WolframAlpha => wolfram::handle(&args),
        Engine::Fast => fast::handle(&args),
        Engine::Local => local::handle(&args)
    }
}
