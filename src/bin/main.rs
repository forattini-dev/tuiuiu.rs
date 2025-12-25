//! Tuiuiu CLI

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "storybook" => run_storybook(),
        "mcp" => run_mcp(),
        "version" | "-v" | "--version" => print_version(),
        "help" | "-h" | "--help" => print_help(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}

fn print_version() {
    println!("tuiuiu {}", env!("CARGO_PKG_VERSION"));
}

fn print_help() {
    println!("🐦 Tuiuiu - Terminal UI Framework");
    println!();
    println!("USAGE:");
    println!("    tuiuiu <COMMAND>");
    println!();
    println!("COMMANDS:");
    println!("    storybook    Run the component storybook");
    println!("    mcp          Start the MCP server");
    println!("    version      Print version info");
    println!("    help         Print this help");
}

fn run_storybook() {
    println!("🎨 Starting Tuiuiu Storybook...");
    println!("(Not yet implemented)");
}

fn run_mcp() {
    println!("🤖 Starting Tuiuiu MCP Server...");
    println!("(Not yet implemented)");
}
