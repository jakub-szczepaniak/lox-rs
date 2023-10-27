use std::env::args;
use std::io::{BufRead,  self};
use std::fmt;

fn main() {
    let args: Vec<String> = args().collect();
    println!("Hello, world! {:?}", args);
    
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
    
    
    match run(&contents) {
        Ok(_) => (),
        Err(e) => {
            report(e, "".to_string());
            std::process::exit(65);
        }
    }
}
fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match run(&line) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),   
        }
        print!("> ");
    }
}

fn run(source: &str) -> Result<(),LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

#[derive(Debug)]
struct Token {
    token: String,
}

struct Scanner{
    source: String,
}
impl Scanner {
    fn new (source: &str) -> Scanner {
        Scanner {
            source: source.to_string(),
        }
    }
    fn scan_tokens(self) -> Vec<Token> {
        Vec::<Token>::new()
    }
}



pub struct LoxError {
    pub message: String,
    pub line: usize,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line: {}, Error: {}", self.line, self.message)
    }
}

pub fn error(line: usize, message: String) -> LoxError {
    LoxError { message, line }
}

pub fn report(err: LoxError, loc: String) {
    println!("[line {}] Error {}: {}", err.line, loc, err.message);
}