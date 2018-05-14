#![feature(nll)]
#![feature(box_syntax, box_patterns)]

use std::io;
use std::io::Write;

#[macro_use]
extern crate lazy_static;
extern crate nix;
extern crate matches;

mod builtin;
mod interpreter;
mod lexer;
mod parser;

use builtin::*;
use interpreter::interpret;
use lexer::Tokenizer;
use parser::Parser;

fn main() {
    loop {
        print!("{}", get_prompt());
        io::stdout().flush().expect("Failed to flush");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // put it to lexer
        match input.as_str().tokenize() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens.iter());
                match parser.parse() {
                    Ok(Some(expr)) => {
                        if let Err(e) = interpret(*expr) {
                            println!("Error executing: {}", e);
                        }
                    }

                    Ok(None) => (),
                    Err(e) => println!("Error in parsing: {}", e),
                };
            }
            Err(e) => println!("Error in lexing: {}", e),
        }
    }
}
