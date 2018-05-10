use std::borrow::Cow;
use std::collections::VecDeque;
use std::mem;
use std::vec;

lazy_static! {
    static ref SYMBOLS: vec::Vec<&'static str> = {
        let mut m = vec!["&&", ";", "&", "|", ">", ">>", "<", "<<", "||", "\n"];

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
    QuotedString(Cow<'a, str>),
    VarString(Cow<'a, str>), // string slice representing commands, parameters to commands, etc
}

pub trait TokenIter<'a>: Iterator<Item = &'a Token<'a>> + Clone {}
impl<'a, T: Iterator<Item = &'a Token<'a>> + Clone> TokenIter<'a> for T {}

impl<'a> Default for Token<'a> {
    fn default() -> Token<'a> {
        Token::WhiteSpace
    }
}

impl<'a> Token<'a> {
    fn new_quotedstring<T: Into<Cow<'a, str>>>(x: T) -> Self {
        Token::QuotedString(x.into())
    }

    fn new_varstring<T: Into<Cow<'a, str>>>(x: T) -> Self {
        Token::VarString(x.into())
    }
}

pub struct LexicalAnalyzer<'a> {
    token_list: VecDeque<Token<'a>>,
}

impl<'a> LexicalAnalyzer<'a> {
    pub fn new() -> LexicalAnalyzer<'a> {
        LexicalAnalyzer {
            token_list: VecDeque::new(),
        }
    }

    // join literals, flatten glob characters
    fn flatten(&mut self) {
        // turn Token::QuotedString into Token::VarString
        for tok in self.token_list.iter_mut() {
            if let Token::QuotedString(_) = *tok {
                let quoted_token = mem::replace(tok, Token::default());
                if let Token::QuotedString(s) = quoted_token {
                    *tok = Token::VarString(s);
                }
            }
        }

        // stitch adjacent VarStrings without seperation characters
        let mut i = 0;
        while i != self.token_list.len() - 1 {
            match (&self.token_list[i], &self.token_list[i + 1]) {
                (Token::VarString(_), Token::VarString(_)) => {
                    let m = self.token_list.remove(i + 1).unwrap();

                    if let Token::VarString(m) = m {
                        if let Token::VarString(ref mut s) = self.token_list[i] {
                            s.to_mut().push_str(&m);
                        }
                    }
                }
                _ => i += 1,
            }
        }

        // remove newline and whitespace
        self.token_list
            .retain(|tok| *tok != Token::WhiteSpace && *tok != Token::Symbol("\n"));
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
                    '\\' => {
                        if let Some((_, ch)) = it.next() {
                            current_token = Some(Token::new_varstring(ch.to_string()));
                        }
                    }
                    '\t' | ' ' => {
                        current_token = Some(Token::WhiteSpace);
                    }
                    '"' | '\'' => {
                        // extract string literal in between quotes
                        let c = it.position(|(_, _ch)| _ch == ch).unwrap();
                        let (start, end) = (i + 1, i + c + 1);
                        current_token = Some(Token::new_quotedstring(&string[start..end]));
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
                        .push_back(Token::new_varstring(&string[start..end]));
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
                        .push_back(Token::new_varstring(&string[start..]));
                }
                break;
            }
        }

        self.flatten();
    }

    pub fn token_iter(&self) -> impl TokenIter {
        self.token_list.iter().clone()
    }
}

#[test]
fn test_analyzer() {
    let string = "  echo void & sleep 1000h; echo '%^;'";
    println!("String: {}", string);
    let mut lexer = LexicalAnalyzer::new();
    lexer.tokenize(&string);

    let mut it = lexer.token_list.iter();
    assert!(it.next() == Some(&Token::new_varstring("echo")));
    assert!(it.next() == Some(&Token::new_varstring("void")));
    assert!(it.next() == Some(&Token::Symbol("&")));
    assert!(it.next() == Some(&Token::new_varstring("sleep")));
    assert!(it.next() == Some(&Token::new_varstring("1000h")));
    assert!(it.next() == Some(&Token::Symbol(";")));
    assert!(it.next() == Some(&Token::new_varstring("echo")));
}

#[test]
fn test_symbol_presence() {
    let string = "<< Help";
    assert!(starts_with_symbol(string) == Some("<<"));
    assert!(starts_with_symbol(&string[2..]) == None);
}
