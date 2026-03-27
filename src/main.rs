use jshape::analyze_json;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut show_examples = true;
    let mut file_path: Option<String> = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--no-examples" => show_examples = false,
            "-h" | "--help" => {
                println!("Usage: jshape [OPTIONS] [FILE]");
                println!();
                println!("Arguments:");
                println!("  [FILE]  JSON file to process (stdin if not provided)");
                println!();
                println!("Options:");
                println!("  --no-examples  Show types instead of example values");
                println!("  -h, --help     Show this help message");
                return;
            }
            _ => {
                if !arg.starts_with('-') {
                    file_path = Some(arg.clone());
                }
            }
        }
    }

    let json_str = if let Some(path) = file_path {
        std::fs::read_to_string(&path).expect("Failed to read file")
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read stdin");
        buffer
    };

    let output = analyze_json(&json_str, show_examples).expect("Invalid JSON input");
    println!("{}", output);
}
