// mod ast_print;
mod error;
mod parser;
mod scanner;
mod token;
mod token_type;
use error::LoxResult;
use scanner::Scanner;
use std::env::args;
use std::io::{self, stdout, BufRead, Write};

use crate::interpreter::Interpreter;
mod callable;
mod environment;
mod expr;
mod interpreter;
mod literal;
mod native_functions;
mod stmt;

fn main() {
    let args: Vec<String> = args().collect();
    let lox = Lox::new();
    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]),
        _ => {
            println!("Usage: lox-rs [script]");
            std::process::exit(64);
        }
    }
}

struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            interpreter: Interpreter::new(),
        }
    }
    pub fn run_file(&self, path: &str) {
        let contents = match std::fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(e) => {
                println!("Error reading file: {}", e);
                std::process::exit(64);
            }
        };

        match self.run(contents) {
            Ok(_) => (),
            Err(_e) => {
                std::process::exit(65);
            }
        }
    }
    pub fn run_prompt(&self) {
        print!("> ");
        let _ = stdout().flush();

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    continue;
                }
                match self.run(line) {
                    Ok(_) => (),
                    Err(_e) => {
                        // error was already reported in scanner
                    }
                }
            } else {
                break;
            }
            print!("> ");
            let _ = stdout().flush();
        }
    }
    fn run(&self, source: String) -> Result<(), LoxResult> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = parser::Parser::new(tokens);
        let statements = parser.parse()?;

        if parser.success() {
            self.interpreter.interprete(&statements);
        }
        Ok(())
    }
}
