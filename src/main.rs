#![feature(nll)]
#![feature(box_syntax, box_patterns)]
#![feature(rust_2018_preview)]

#![warn(rust_2018_idioms)]

use std::env;
use std::io;
use std::io::Write;

mod builtin;
mod interpreter;
mod lexer;
mod parser;

use crate::builtin::*;
use crate::interpreter::interpret;
use crate::lexer::Tokenizer;
use crate::parser::Parser;

fn main() {
    set_shell_signal_handlers();

    loop {
        print!("{}", get_prompt());
        io::stdout().flush().expect("Failed to flush");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // end of stream
        if input.is_empty() {
            return;
        }

        let debug_print = env::var("DEBUG_PRINT").is_ok();

        // put it to lexer
        match input.as_str().tokenize() {
            Ok(tokens) => {
                if debug_print {
                    println!("Tokens: {:?}", &tokens);
                }
                let mut parser = Parser::new(tokens.iter());
                match parser.parse() {
                    Ok(Some(expr)) => {
                        if debug_print {
                            println!("Syntax Tree: \n{:#?}\n", &expr);
                        }
                        if let Err(e) = interpret(&*expr) {
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
