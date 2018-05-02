#![allow(dead_code)]
use std::io;
use std::io::Write;

mod prompt;
mod lexer;

use prompt::Prompt;
use lexer::LexicalAnalyzer;

fn ex_command(cmd: String) {
    print!("You wrote {}", cmd)
}

fn main() {
    let prompt = Prompt::new(String::from("swoorup % "));
    let mut lexer = LexicalAnalyzer::new();

    loop {
        print!("{}", prompt.get_prompt());
        io::stdout().flush().expect("Failed to flush");

        // read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // put it to lexer
        lexer.analyze(&input);

        ex_command(input);
    }
}
