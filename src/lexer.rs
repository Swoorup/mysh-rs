use std::collections::LinkedList;

#[derive(Debug)]
enum Token <'a> {
    Symbols(String), // i.e ';', '&', etc
    VarString(&'a str), // string slice representing commands, parameters to commands, etc
}

struct LexicalAnalyzer <'a> {
    token_list: LinkedList<Token>,
}

impl LexicalAnalyzer {
    pub fn new() -> LexicalAnalyzer {
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
