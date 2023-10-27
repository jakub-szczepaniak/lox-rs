use std::env::args;
use std::io::{BufRead,  self};

fn main() {
    let args: Vec<String> = args().collect();
    println!("Hello, world! {:?}", args);
    
    if args.len() > 1 {
        println!("Usage: lox-rs [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_prompt();
    } else {
        run_file(&args[1]);
    }

    println!("Hello, world! {:?}", args);
}

fn run_file(path: &str) -> io::Result<()> {
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading file: {}", e);
            return Ok(());
        }
    };
    
    
    run(&contents);
    Ok(())
}
fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        run(&line);
        print!("> ");
    }
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
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
