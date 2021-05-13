//! Lexer module is responsible for splitting a string into tokens

use crate::parser::*;
use std::{collections::VecDeque, fmt, mem};

fn try_extract_symbol_at_start(line: &str) -> Option<&'static str> {
    fn get_symbols() -> [&'static str; 10] {
        // reverse sort for longest match rule
        let mut m =["&&", ";", "&", "|", ">>", "<<", "<", ">", "||", "\n"];
        m.sort_by(|a, b| b.cmp(a));
        m
    }

    get_symbols()
        .iter()
        .find(|&&sym| line.starts_with(sym)).copied()
}

pub struct Tokens<'a> (VecDeque<Token<'a>>);

pub trait Tokenizer {
    fn tokenize(&self) -> Result<Tokens<'_>, String>;
}

impl fmt::Debug for Tokens<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<'a, T> TokenStream<'a> for T 
    where T: Iterator<Item = &'a Token<'a>> + Clone {}

impl Tokens<'_> {
    // join literals, flatten glob characters
    // glob flatten -> NOT IMPLEMENTED
    fn flatten(mut self) -> Self {
        // turn Token::QuotedString into Token::VarString
        for tok in &mut self.0 {
            if let Token::QuotedString(_) = *tok {
                let quoted_token = mem::replace(tok, Token::default());
                if let Token::QuotedString(s) = quoted_token {
                    *tok = Token::VarString(s);
                }
            }
        }

        // stitch adjacent VarStrings without seperation characters
        let mut i = 0;
        while !self.0.is_empty() && i != self.0.len() - 1 {
            match (&self.0[i], &self.0[i + 1]) {
                (Token::VarString(_), Token::VarString(_)) => {
                    let m = self.0.remove(i + 1).unwrap();

                    if let Token::VarString(m) = m {
                        if let Token::VarString(ref mut s) = self.0[i] {
                            s.to_mut().push_str(&m);
                        }
                    }
                }
                _ => i += 1,
            }
        }

        // remove newline and whitespace
        self.0
            .retain(|tok| *tok != Token::WhiteSpace && *tok != Token::Symbol("\n"));

        self
    }

    pub fn get_stream(&self) -> impl TokenStream<'_>{
        self.0.iter()
    }
}

impl Tokenizer for str {
    fn tokenize(&self) -> Result<Tokens<'_>, String> {
        let mut tokens: VecDeque<Token<'_>> = VecDeque::new();
        let mut it = self.chars().enumerate().peekable();

        let mut start = 0;
        let mut capture_state = false;

        while let Some((i, ch)) = it.next() {
            let current_token = match ch {
                '\\' => {
                    if let Some((_, ch)) = it.next() {
                        Some(Token::VarString(ch.to_string().into()))
                    } else {
                        None
                    }
                }
                '\t' | ' ' => Some(Token::WhiteSpace),
                '"' | '\'' => {
                    // extract string literal in between quotes
                    let c = it.position(|(_, _ch)| _ch == ch).ok_or("cannot find endin quote")?;
                    let (start, end) = (i + 1, i + c + 1);
                    Some(Token::QuotedString((&self[start..end]).into()))
                }
                _ => {
                    let remaining_str = &self[i..];
                    if let Some(s) = try_extract_symbol_at_start(remaining_str) {
                        Some(Token::Symbol(s))
                    } else {
                        if !capture_state {
                            capture_state = true;
                            start = i;
                        }
                        None
                    }
                }
            };

            if capture_state && current_token.is_some() {
                capture_state = false;
                let end = i;
                tokens.push_back(Token::VarString((&self[start..end]).into()));
            }

            match current_token {
                Some(Token::WhiteSpace) => {
                    // ignore duplicates whitespace
                    if !tokens.is_empty() && tokens.back() != Some(&Token::WhiteSpace) {
                        tokens.push_back(Token::WhiteSpace);
                    }
                }
                Some(tok) => {
                    tokens.push_back(tok);
                }
                None => (),
            }
        }
        if capture_state {
            tokens.push_back(Token::VarString((&self[start..]).into()));
        }

        Ok(Tokens(tokens).flatten())
    }
}

#[test]
fn test_tokenizer() {
    use std::borrow::Cow;

    let tokens = " echo void &'sle''ep' 1000h;echo '%^;'".tokenize().unwrap();
    println!("{:?}", tokens);

    let mut it = tokens.get_stream();
    assert!(it.next() == Some(&Token::VarString(Cow::from("echo"))));
    assert!(it.next() == Some(&Token::VarString(Cow::from("void"))));
    assert!(it.next() == Some(&Token::Symbol("&")));
    assert!(it.next() == Some(&Token::VarString(Cow::from("sleep"))));
    assert!(it.next() == Some(&Token::VarString(Cow::from("1000h"))));
    assert!(it.next() == Some(&Token::Symbol(";")));
    assert!(it.next() == Some(&Token::VarString(Cow::from("echo"))));
}

#[test]
fn test_symbol_presence() {
    let string = "<< Help";
    println!("starts with {:?}", try_extract_symbol_at_start(string));
    assert!(try_extract_symbol_at_start(string) == Some("<<"));
    assert!(try_extract_symbol_at_start(&string[2..]) == None);
}
