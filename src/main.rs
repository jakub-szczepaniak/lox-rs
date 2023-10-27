use std::env::args;
use std::io::{Read, BufReader};
use std::fs::File;

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

fn run_file(path: &str) {
    let file = File::open(path).expect("file not found");
    let mut buf_reader = BufReader::new(file); 
    let mut buf = Vec::<u8>::new();

    buf_reader.read_to_end(&mut buf).expect("error reading file");

    run(&buf);

}
fn run_prompt() {}

fn run(source: &[u8]) {
    println!("run: {:?}", source);
}
