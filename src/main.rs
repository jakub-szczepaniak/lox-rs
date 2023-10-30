mod error;
mod token_type;
mod token;
mod scanner;
use scanner::Scanner;
use error::LoxError;
use token::Token;
use std::env::args;
use std::io::{BufRead,  self};

fn main() {
    let args: Vec<String> = args().collect();
    
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => { 
            println!("Usage: lox-rs [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) {
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading file: {}", e);
            std::process::exit(64); 
        }
    };
    
    
    match run(contents) {
        Ok(_) => (),
        Err(e) => {
            e.report("".to_string());
            std::process::exit(65);
        }
    }
}
fn run_prompt() {
    print!("> ");
   
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                continue;
            } 
            match run(line) {
                Ok(_) => (),
                Err(e) => {
                    e.report("".to_string());
                }
            }

        } else {
            break;
        }
    print!("> ");
   
    }
}

fn run(source: String) -> Result<(),LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}





