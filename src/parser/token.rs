use std::borrow::Cow;

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
    pub fn new_varstring(string: &'a str) -> Token<'a> {
        Self::VarString(Cow::Borrowed(string))
    }

    pub fn varstring(&self) -> Option<String> {
        match self {
            Token::VarString(s) => Some(s.to_string()),
            _ => None,
        }
    }

    pub fn symbol(&self) -> Option<&'static str> {
        match self {
            Token::Symbol(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_varstring(&self) -> bool {
        if let Token::VarString(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_symbol(&self) -> bool {
        if let Token::Symbol(_) = self {
            true
        } else {
            false
        }
    }
}