use std::collections::LinkedList;
use std::vec;

lazy_static! {
    static ref SYMBOLS: vec::Vec<&'static str> = {
        let mut m = vec!["&&", ";", "&", "|", ">", ">>", "<", "<<", "||"];
    
        // sort for longest match rule
        m.sort_by(|a, b| b.cmp(a));
        m
    };
}

fn starts_with_symbol(line: &str) -> Option<String> {
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

    pub fn tokenize(&mut self, string: &'a str) {
        let mut it = string.chars().enumerate().peekable();

        while let Some((i, ch)) = it.next() {
            match ch {
                '\t' | ' ' => {
                    // ignore if last token was whitespace
                    match self.token_list.back() {
                        Some(&Token::WhiteSpace) => (),
                        _ => self.token_list.push_back(Token::WhiteSpace),
                    }
                }
                '"' | '\'' => {
                    // extract string literal in between quotes
                    let c = it.position(|(_, _ch)| _ch == ch).unwrap();
                    let (start, end) = (i+1, i+c+1);
                    self.token_list.push_back(Token::VarString(&string[start..end]));
                }
                _ => {
                    let remaining_str = &string[i..];
                    if let Some(s) = starts_with_symbol(remaining_str) {
                        let symbol_length = s.len();
                        self.token_list.push_back(Token::Symbols(String::from(s)));
                        for _ in 0..symbol_length {
                            it.next();
                        }
                    } else {
                        println!("{}", &remaining_str);
                    }
                },
            } 
        }
    }
}

#[test]
fn test_analyzer() {
    let string = String::from("Hello World, you are \"sometimes\" 'ok' && sometimes not!!!");
    let mut lexer = LexicalAnalyzer::new();
    lexer.tokenize(&string);
    println!("Token list: {:?}", lexer.token_list);
}

#[test]
fn test_symbol_presence() {
    let string = "<< Help";
    match starts_with_symbol(string) {
        Some(x) => println!("{}", x),
        None => (),
    }
}