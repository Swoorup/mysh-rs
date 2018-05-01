#![allow(dead_code)]
use std::io;
use std::io::Write;

mod prompt;

use prompt::Prompt;

fn ex_command(cmd: String) {
    print!("You wrote {}", cmd)
}

fn main() {
    let prompt = Prompt::new(String::from("swoorup % "));

    loop {
        print!("{}", prompt.get_prompt());
        io::stdout().flush().expect("Failed to flush");

        // read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        ex_command(input);
    }
}
