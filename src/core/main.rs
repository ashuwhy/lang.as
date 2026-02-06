use std::env;
use std::fs;
use std::process;

use aslang::runtime;

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
            } else {
                run_file(&args[1]);
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
    println!("Type 'exit' to quit");
    
    loop {
        print!("as > ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        if input == "exit" {
            break;
        }
        
        match runtime::execute(&input) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn run_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => {
            match runtime::execute(&contents) {
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