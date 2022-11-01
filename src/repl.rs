use std::io::{stdin, stdout, Write};

use crate::{environment::Environment, eval::eval, lexer::Lexer, parser::Parser};

const PROMPT: &'static str = ">> ";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        stdout().flush().unwrap();

        let mut line = String::new();

        stdin().read_line(&mut line).expect("Failed to read line");

        let lexer = Lexer::new(line);
        let mut parser = Parser::new(lexer);
        let node = match parser.parse() {
            Ok(n) => n,
            _ => continue,
        };
        let result = match eval(node, &Environment::new()) {
            Ok(n) => n,
            _ => continue,
        };

        println!("{}", result);
    }
}
