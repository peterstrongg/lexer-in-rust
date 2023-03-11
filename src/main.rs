use std::env;
use std::io;
use std::fs;
use std::io::Write; // flush, unwrap

mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: {} <filename>", &args[0]);
        return;
    } else if args.len() < 2 {
        repl();
        return;
    }

    let src: String = fs::read_to_string(&args[1])
        .expect("Error reading file");

    let mut scanner = utils::scanner::Scanner::new(src);
    scanner.scan_for_tokens();
}

fn repl() {   
    println!("Synapse version 0.0.1\n");
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    }
}
