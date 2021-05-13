#![feature(nll)]
#![feature(box_syntax, box_patterns)]
#![feature(associated_type_defaults)]
#![warn(rust_2018_idioms)]

use std::env;
use std::io;
use std::io::Write;

mod builtin;
mod interpret;
mod lexer;
mod parser;

use crate::{lexer::Tokenizer, parser::Parse};

fn main() {
    builtin::set_shell_signal_handlers();

    loop {
        print!("{}", builtin::get_prompt());
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
                let parsed_result = tokens.get_stream().parse();
                match parsed_result {
                    Ok(Some(expr)) => {
                        if debug_print {
                            println!("Syntax Tree: \n{:#?}\n", &expr);
                        }
                        if let Err(e) = interpret::interpret(&*expr) {
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
