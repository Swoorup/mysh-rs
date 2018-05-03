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
        let mut bInStrToken = false;

        loop {
            match it.peek() {
                Some(&(i, ch)) => match ch {
                    '\t' | ' ' => println!("Found character at {}", i),
                    '\\' => 
                    q_chr @ '"' | q_chr @ '\''  => {
                        // extract string literal in between quotes
                        it.next();
                        let c = it.position(|(_, ch)| ch == q_chr).unwrap();
                        let (start, end) = (i+1, i+c+1);
                        self.token_list.push_back(Token::VarString(&string[start..end]));
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
    let string = String::from("Hey sexy lady, you are \"ugly\" \"jk\"");
    let mut lexer = LexicalAnalyzer::new();
    lexer.analyze(&string);
}