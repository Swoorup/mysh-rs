#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(nll)]

use std::io;
use std::io::Write;

#[macro_use] extern crate lazy_static;

mod prompt;
mod lexer;
mod parser;

use prompt::Prompt;
use lexer::LexicalAnalyzer;
use parser::Parser;

fn main() {
    let prompt = Prompt::new(String::from("swoorup % "));

    loop {
        print!("{}", prompt.get_prompt());
        io::stdout().flush().expect("Failed to flush");

        // read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // put it to lexer
        let mut lexer = LexicalAnalyzer::new();
        lexer.tokenize(&input);

        // use iter
        let mut parser = Parser::new();
        match parser.parse(lexer.token_iter()) {
            Ok(_) => (),
            Err(e) => println!("Error in parsing: {}", e),
        }
    }
}
