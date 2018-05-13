#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(nll)]
#![feature(box_syntax, box_patterns)]

use std::io;
use std::io::Write;

#[macro_use]
extern crate lazy_static;
extern crate nix;

mod builtin;
mod interpreter;
mod lexer;
mod parser;

use builtin::*;
use interpreter::interpret;
use lexer::LexicalAnalyzer;
use parser::Parser;

fn main() {
    loop {
        print!("{}", get_prompt());
        io::stdout().flush().expect("Failed to flush");

        // read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // put it to lexer
        let mut lexer = LexicalAnalyzer::new();
        lexer.tokenize(&input);

        // use iter
        let mut parser = Parser::new(lexer.token_iter());
        match parser.parse() {
            Ok(Some(expr)) => interpret(*expr),
            Ok(None) => (),
            Err(e) => println!("Error in parsing: {}", e),
        }
    }
}
