use std::env;
use std::fs;
use std::process;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use aslang::runtime::Runtime;

mod lsp;

const VERSION: &str = "0.1.0";
const AUTHOR: &str = "Ashutosh Sharma <ashutoshsharmawhy@gmail.com>";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => start_repl(),
        2 => {
            if args[1] == "--version" {
                println!("aslang version {}", VERSION);
                println!("Author: {}", AUTHOR);
            } else if args[1] == "--repl" {
                start_repl();
            } else if args[1] == "lsp" {
                if let Err(e) = lsp::start_lsp() {
                    eprintln!("LSP Error: {}", e);
                    process::exit(1);
                }
            } else {
                run_file(&args[1], false);
            }
        }
        3 => {
            if args[1] == "--debug" {
                run_file(&args[2], true);
            } else {
                println!("Usage: aslang [filename.as] or aslang --debug [filename.as]");
                process::exit(1);
            }
        }
        _ => {
            println!("Usage: aslang [filename.as]");
            process::exit(1);
        }
    }
}

fn start_repl() {
    println!("ASLang {} - Interactive Mode", VERSION);
    println!("Type 'exit' or Ctrl-D to quit");
    
    let mut runtime = Runtime::new();
    let mut rl = DefaultEditor::new().unwrap();
    if rl.load_history("history.txt").is_err() {
        // No previous history
    }

    loop {
        let readline = rl.readline("as > ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                let _ = rl.add_history_entry(line.as_str()); // Add raw line to history
                
                if input == "exit" {
                    break;
                }
                if input.is_empty() {
                    continue;
                }
                
                match runtime.execute(input) {
                    Ok(result) => {
                        if !result.is_empty() {
                           println!("{}", result);
                        }
                    },
                    Err(e) => eprintln!("Error: {}", e),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    // Ignore error if saving history fails
    let _ = rl.save_history("history.txt");
}

fn run_file(filename: &str, debug: bool) {
    match fs::read_to_string(filename) {
        Ok(contents) => {
            let mut runtime = Runtime::new();
            runtime.debug = debug;
            match runtime.execute(&contents) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Runtime error: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Could not read file '{}': {}", filename, e);
            process::exit(1);
        }
    }
}