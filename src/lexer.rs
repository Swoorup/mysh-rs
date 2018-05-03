use std::collections::LinkedList;
use std::iter::Enumerate;
use std::iter::Iterator;
use std::iter::Peekable;
use std::str::Chars;
use std::vec;

lazy_static! {
    static ref SYMBOLS: vec::Vec<&'static str> = {
        let mut m = vec!["&&", ";", "&", "|", ">", ">>", "<", "<<", "||"];
    
        // sort to allow longest match rule
        m.sort_by(|a, b| b.cmp(a));
        m
    };
}

fn begins_with_symbol(line: &str) -> Option<String> {
    for symbol in SYMBOLS.iter() {
        if line.starts_with(symbol) {
            return Some(String::from(*symbol));
        }
    }
    None
}

#[derive(Debug)]
enum Token <'a> {
    WhiteSpace,
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
                Some(&(i, ch)) => match ch {
                    '\t' | ' ' => {
                        // ignore if last token was whitespace
                        match self.token_list.back() {
                            Some(&Token::WhiteSpace) => (),
                            _ => self.token_list.push_back(Token::WhiteSpace),
                        }
                    }
                    '"' | '\''  => {
                        // extract string literal in between quotes
                        it.next();
                        let c = it.position(|(_, _ch)| _ch == ch).unwrap();
                        let (start, end) = (i+1, i+c+1);
                        self.token_list.push_back(Token::VarString(&string[start..end]));
                    }
                    _    => {
                    },
                }
                None => break,
            }
            it.next();
        }
        println!("Token list: {:?}", self.token_list);
    }
}

#[test]
fn test_analyzer() {
    let string = String::from("Hey sexy lady, you are \"ugly\" \"jk\"");
    let mut lexer = LexicalAnalyzer::new();
    lexer.analyze(&string);
}

#[test]
fn test_symbol_presence() {
    let string = "<< Help";
    match begins_with_symbol(string) {
        Some(x) => println!("{}", x),
        None => (),
    }
}