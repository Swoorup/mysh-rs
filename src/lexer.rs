use std::collections::LinkedList;

#[derive(Debug)]
enum Token <'a> {
    Symbols(String), // i.e ';', '&', etc
    VarString(&'a str), // string slice representing commands, parameters to commands, etc
}

pub struct LexicalAnalyzer <'a> {
    token_list: LinkedList<Token<'a>>,
}

impl <'a> LexicalAnalyzer <'a> {
    pub fn new() -> LexicalAnalyzer<'a> {
        LexicalAnalyzer {
            token_list: LinkedList::new(), 
        }
    }

    pub fn analyze(&mut self, string: &String) {
        let mut it = string.chars().enumerate();

        loop {
            match it.next() {
                Some((i, ch)) => println!("Found {} at {}", ch, i),
                None => break,
            }
        }
    }
}

#[test]
fn test_analyzer() {
    let mut lexer = LexicalAnalyzer::new();
    lexer.analyze(&String::from("Hey sexy lady"));
}
