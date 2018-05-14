use parser::*;
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

#[derive(Debug)]
pub struct TokenContainer<'a> {
    token_list: VecDeque<Token<'a>>,
}

impl<'a> TokenContainer<'a> {
    // join literals, flatten glob characters
    // glob flatten -> NOT IMPLEMENTED
    fn flatten(mut self) -> Self {
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
        while !self.token_list.is_empty() && i != self.token_list.len() - 1 {
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

        self
    }

    pub fn iter(&self) -> impl TokenIter {
        self.token_list.iter()
    }
}

pub trait Tokenizer {
    fn tokenize(&self) -> Result<TokenContainer, String>;
}

impl Tokenizer for str {
    fn tokenize(&self) -> Result<TokenContainer, String> {
        let mut token_list: VecDeque<Token> = VecDeque::new();
        let mut it = self.chars().enumerate().peekable();

        let mut start = 0;
        let mut capture_state = false;

        while let Some((i, ch)) = it.next() {
            let current_token: Option<Token> = match ch {
                '\\' => {
                    if let Some((_, ch)) = it.next() {
                        Some(Token::new_varstring(ch.to_string()))
                    } else {
                        None
                    }
                }
                '\t' | ' ' => Some(Token::WhiteSpace),
                '"' | '\'' => {
                    // extract string literal in between quotes
                    let c = it.position(|(_, _ch)| _ch == ch).ok_or("cannot find endin quote")?;
                    let (start, end) = (i + 1, i + c + 1);
                    Some(Token::new_quotedstring(&self[start..end]))
                }
                _ => {
                    let remaining_str = &self[i..];
                    if let Some(s) = starts_with_symbol(remaining_str) {
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
                token_list.push_back(Token::new_varstring(&self[start..end]));
            }

            match current_token {
                Some(Token::WhiteSpace) => {
                    // ignore duplicates whitespace
                    if !token_list.is_empty() && token_list.back() != Some(&Token::WhiteSpace) {
                        token_list.push_back(Token::WhiteSpace);
                    }
                }
                Some(tok) => {
                    token_list.push_back(tok);
                }
                None => (),
            }
        }
        if capture_state {
            token_list.push_back(Token::new_varstring(&self[start..]));
        }

        Ok(TokenContainer {
            token_list: token_list,
        }.flatten())
    }
}

#[test]
fn test_tokenizer() {
    let tokens = " echo void &'sle''ep' 1000h;echo '%^;'".tokenize().unwrap();
    println!("{:?}", tokens);

    let mut it = tokens.iter();
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
