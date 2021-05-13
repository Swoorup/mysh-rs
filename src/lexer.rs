use crate::parser::*;
use std::{collections::VecDeque, fmt, mem};

const fn get_symbols() -> [&'static str; 10] {
    // reverse sort for longest match rule
    // m.sort_by(|a, b| b.cmp(a));
    ["&&", ";", "&", "|", ">>", "<<", "<", ">", "||", "\n"]
}

const SYMBOLS:[&str; 10] = get_symbols();

fn starts_with_symbol(line: &str) -> Option<&'static str> {
    SYMBOLS
        .iter()
        .find(|&&sym| line.starts_with(sym)).copied()
}

pub struct TokenContainer<'a> {
    token_list: VecDeque<Token<'a>>,
}

impl fmt::Debug for TokenContainer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.token_list)
    }
}

impl<'a, T: Iterator<Item = &'a Token<'a>> + Clone> TokenStream<'a> for T {}

impl TokenContainer<'_> {
    // join literals, flatten glob characters
    // glob flatten -> NOT IMPLEMENTED
    fn flatten(mut self) -> Self {
        // turn Token::QuotedString into Token::VarString
        for tok in &mut self.token_list {
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

    pub fn get_stream(&self) -> impl TokenStream<'_>{
        self.token_list.iter()
    }
}

pub trait Tokenizer {
    fn tokenize(&self) -> Result<TokenContainer<'_>, String>;
}

impl Tokenizer for str {
    fn tokenize(&self) -> Result<TokenContainer<'_>, String> {
        let mut token_list: VecDeque<Token<'_>> = VecDeque::new();
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
                token_list.push_back(Token::VarString((&self[start..end]).into()));
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
            token_list.push_back(Token::VarString((&self[start..]).into()));
        }

        Ok(TokenContainer {
            token_list,
        }.flatten())
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
    println!("starts with {:?}", starts_with_symbol(string));
    assert!(starts_with_symbol(string) == Some("<<"));
    assert!(starts_with_symbol(&string[2..]) == None);
}
