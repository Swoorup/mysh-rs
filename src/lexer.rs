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

    pub fn analyze(&mut self, string: &'a String) {
        let mut it = string.chars().enumerate().peekable();

        loop {
            match it.peek() {
                Some((i, ch)) => match ch {
                    '\t' => println!("Found tab character"),
                    ' '  => println!("Found whitespace"),
                    '"'  => {
                        let end_pos = it.position(|(_, ch)| ch == '"').unwrap();
                        self.token_list.push_back(Token::VarString(&string[*i..end_pos]));
                        println!("Found string literal: {:?}", self.token_list);
                    }
                    _    => (),
                }
                None => break,
            }
            it.next();
        }
    }
}

#[test]
fn test_analyzer() {
    let string = String::from("Hey sexy lady");
    let mut lexer = LexicalAnalyzer::new();
    lexer.analyze(&string);
}