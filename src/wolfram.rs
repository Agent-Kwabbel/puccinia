use std::env;
use crate::cli::Cli;
use reqwest::blocking::get;
use reqwest::StatusCode;
use std::io::copy;
use std::process::Command;
use tempfile::NamedTempFile;
use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct QueryResult {
    #[serde(rename = "pod")]
    pods: Vec<Pod>,
}

#[derive(Debug, Deserialize)]
struct Pod {
    #[serde(rename = "@title")]
    title: String,

    #[serde(rename = "subpod")]
    subpods: Vec<Subpod>,
}

#[derive(Debug, Deserialize)]
struct Subpod {
    plaintext: Option<String>,
}

pub fn handle(args: &Cli) {
    let input = &args.search;

    let use_short = args.short || (!args.simple && !args.full);
    
    if args.simple {
        match env::var("WOLFRAM_API_SIMPLE") {
            Ok(url) => {
                let full_url = format!("{}{}", url, urlencoding::encode(input));
                println!("Downloading Wolfram Alpha image...");

                match get(&full_url) {
                    Ok(mut response) => {
                        if response.status().is_success() {
                            let mut temp_file = NamedTempFile::new().expect("Couldn't create temp file");
                            copy(&mut response, &mut temp_file).expect("Failed to write image to temp file");

                            let path = temp_file.into_temp_path();
                            let path_str = path.to_str().unwrap();

                            let open_result = Command::new("loupe") 
                                .arg(path_str)
                                .status();

                            if let Err(e) = open_result {
                                eprintln!("Failed to open image viewer: {}", e);
                                eprintln!("You can manually open the file at: {}", path_str);
                            }
                        } else {
                            eprintln!("Failed to fetch image. Status: {}", response.status());
                        }
                    }
                    Err(e) => {
                        eprintln!("→ HTTP request failed: {}", e);
                    }
                }
            }
            Err(_) => {
                eprintln!("Missing env var: WOLFRAM_API_SIMPLE");
            }
        }

        return;
    }

    let (label, base_url) = if use_short {
        ("Short", env::var("WOLFRAM_API_SHORT"))
    } else if args.full {
        ("Full", env::var("WOLFRAM_API_FULL"))
    } else {
        eprintln!("No Wolfram Alpha mode selected (use --simple, --short, or --full)");
        return;
    };

    match base_url {
        Ok(url) => {
            let full_url = format!("{}{}", url, urlencoding::encode(input));
            println!("Querying Wolfram Alpha [{}]...", label);

            match get(&full_url) {
                Ok(response) => {
                    match response.status() {
                        StatusCode::OK => {
                            if let Ok(body) = response.text() {
                                if args.full {
                                    match from_str::<QueryResult>(&body) {
                                        Ok(parsed) => {
                                            for pod in parsed.pods {
                                                println!("\n# {}", pod.title);
                                                for subpod in pod.subpods {
                                                    if let Some(text) = subpod.plaintext {
                                                        if !text.trim().is_empty() {
                                                            println!("{}", text);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("❌ Failed to parse XML: {}", e);
                                            println!("Raw response:\n{}", body);
                                        }
                                    }
                                } else {
                                    println!("{}", body);
                                }
                            } else {
                                eprintln!("Could not read response body.");
                            }
                        }
                        _ => {
                            eprintln!("→ Error: Received HTTP {}", response.status());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("→ HTTP request failed: {}", e);
                }
            }
        }
        Err(_) => {
            eprintln!("Missing env var for selected mode: {}", label);
        }
    }
}

