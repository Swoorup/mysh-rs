use std::collections::VecDeque;

use lexer::Token;

#[derive(Debug)]
struct SyntaxTree<'a> {
    left: Option<Box<SyntaxTree<'a>>>,
    right: Option<Box<SyntaxTree<'a>>>,
    token: &'a Token<'a>,
}

impl<'a> SyntaxTree<'a> {
    fn new(token: &'a Token) -> SyntaxTree<'a> {
        SyntaxTree {
            left: None,
            right: None,
            token: token,
        }
    }

    fn insert_left(&mut self, token: &'a Token) {
        self.left = Some(Box::new(SyntaxTree {
            left: None,
            right: None,
            token: token,
        }));
    }

    fn insert_right(&mut self, token: &'a Token) {
        self.right = Some(Box::new(SyntaxTree {
            left: None,
            right: None,
            token: token,
        }));
    }
}

#[test]
fn test_syntax_tree() {
    let mut syntaxtree = SyntaxTree::new(&Token::WhiteSpace);
    syntaxtree.insert_left(&Token::WhiteSpace);
    syntaxtree.insert_right(&Token::WhiteSpace);
    syntaxtree
        .right
        .as_mut()
        .unwrap()
        .insert_left(&Token::WhiteSpace);
    println!("{:?}", syntaxtree);
}

pub struct Parser<'a> {
    ast: Option<SyntaxTree<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser { ast: None }
    }

    pub fn parse(&mut self, token_list: VecDeque<Token>) -> Result<(), String> {
        for i in token_list {
            // println!("{:?}", i);
        }
        Ok(())
    }
}
