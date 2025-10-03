use crate::cli::Cli;
use meval::eval_str;

pub fn handle(args: &Cli) {
    let input = &args.search;

    match eval_str(input) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("❌ Could not evaluate expression: {}", e),
    }
}

