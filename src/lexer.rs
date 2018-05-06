use std::collections::LinkedList;
use std::vec;

lazy_static! {
    static ref SYMBOLS: vec::Vec<&'static str> = {
        let mut m = vec!["&&", ";", "&", "|", ">", ">>", "<", "<<", "||"];

        // reverse sort for longest match rule
        m.sort_by(|a, b| b.cmp(a));
        m
    };
}

fn starts_with_symbol(line: &str) -> Option<&'static str> {
    SYMBOLS
        .iter()
        .find(|&&sym| line.starts_with(sym))
        .map(|sym| *sym)
}

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    WhiteSpace,
    Symbol(&'static str), // i.e ';', '&', etc
    QuotedString(&'a str),
    VarString(&'a str),    // string slice representing commands, parameters to commands, etc
}

pub struct LexicalAnalyzer<'a> {
    pub token_list: LinkedList<Token<'a>>, // TODO: turn into iter
}

impl<'a> LexicalAnalyzer<'a> {
    pub fn new() -> LexicalAnalyzer<'a> {
        LexicalAnalyzer {
            token_list: LinkedList::new(),
        }
    }

    // join literals, flatten glob characters
    fn flatten(&mut self) {
        // let mut it = self.token_list.iter_mut();

        // while let Some(token) = it.next() {
        //     match token {
        //         t => {
        //             self.token_list.pop_back();
        //             println!("Token {:?}", t);
        //         }
        //     }
        // }
    }

    pub fn tokenize(&mut self, string: &'a str) {
        let mut it = string.chars().enumerate().peekable();

        let mut start = 0;
        let mut capturing = false;

        loop {
            if let Some((i, ch)) = it.next() {
                let mut omit_current_ch_for_capture = true;
                let mut current_token: Option<Token> = None;

                match ch {
                    '\t' | ' ' => {
                        current_token = Some(Token::WhiteSpace);
                    }
                    '"' | '\'' => {
                        // extract string literal in between quotes
                        let c = it.position(|(_, _ch)| _ch == ch).unwrap();
                        let (start, end) = (i + 1, i + c + 1);
                        current_token = Some(Token::QuotedString(&string[start..end]));
                    }
                    _ => {
                        let remaining_str = &string[i..];
                        if let Some(s) = starts_with_symbol(remaining_str) {
                            current_token = Some(Token::Symbol(s));
                            for _ in 1..s.len() {
                                it.next();
                            }
                        } else {
                            omit_current_ch_for_capture = false;
                            if !capturing {
                                capturing = true;
                                start = i;
                            }
                        }
                    }
                }

                if capturing && omit_current_ch_for_capture {
                    capturing = false;
                    let end = i;
                    self.token_list
                        .push_back(Token::VarString(&string[start..end]));
                }

                match current_token {
                    Some(Token::WhiteSpace) => {
                        // ignore duplicates whitespace
                        if !self.token_list.is_empty()
                            && self.token_list.back() != Some(&Token::WhiteSpace)
                        {
                            self.token_list.push_back(Token::WhiteSpace);
                        }
                    }
                    Some(tok) => {
                        self.token_list.push_back(tok);
                    }
                    None => (),
                }
            } else {
                if capturing {
                    self.token_list
                        .push_back(Token::VarString(&string[start..]));
                }
                break;
            }
        }

        self.flatten();
    }
}

#[test]
fn test_analyzer() {
    let string = "  echo void & sleep 1000h; echo '%^;'";
    println!("String: {}", string);
    let mut lexer = LexicalAnalyzer::new();
    lexer.tokenize(&string);

    let mut it = lexer.token_list.iter();
    assert!(it.next() == Some(&Token::VarString("echo")));
    assert!(it.next() == Some(&Token::WhiteSpace));
    assert!(it.next() == Some(&Token::VarString("void")));
    assert!(it.next() == Some(&Token::WhiteSpace));
    assert!(it.next() == Some(&Token::Symbol("&")));
    assert!(it.next() == Some(&Token::WhiteSpace));
    assert!(it.next() == Some(&Token::VarString("sleep")));
    assert!(it.next() == Some(&Token::WhiteSpace));
    assert!(it.next() == Some(&Token::VarString("1000h")));
    assert!(it.next() == Some(&Token::Symbol(";")));
    assert!(it.next() == Some(&Token::WhiteSpace));
    assert!(it.next() == Some(&Token::VarString("echo")));
}

#[test]
fn test_symbol_presence() {
    let string = "<< Help";
    assert!(starts_with_symbol(string) == Some("<<"));
    assert!(starts_with_symbol(&string[2..]) == None);
}
