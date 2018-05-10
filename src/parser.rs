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

//	test all command line production orderwise
fn test_cmdline<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    return if let Some(st) = test_cmdline_1(token_list) {
        Some(st)
    } else if let Some(st) = test_cmdline_2(token_list) {
        Some(st)
    } else if let Some(st) = test_cmdline_3(token_list) {
        Some(st)
    } else if let Some(st) = test_cmdline_4(token_list) {
        Some(st)
    } else if let Some(st) = test_cmdline_5(token_list) {
        Some(st)
    } else {
        None
    };
}
//	<job> ';' <command line>
fn test_cmdline_1<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    None
}
//	<job> ';'
fn test_cmdline_2<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    None
}
//	<job> '&' <command line>
fn test_cmdline_3<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    None
}
//	<job> '&'
fn test_cmdline_4<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    None
}
//	<job>
fn test_cmdline_5<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    None
}

// test all job production in order
fn test_job<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    return if let Some(st) = test_job_1(token_list) {
        Some(st)
    } else if let Some(st) = test_job_2(token_list) {
        Some(st)
    } else {
        None
    };
}
// <command> '|' <job>
fn test_job_1<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    None
}
// <command>
fn test_job_2<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    None
}

// test all command production orderwise
fn test_cmd<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    return if let Some(st) = test_cmd_1(token_list) {
        Some(st)
    } else if let Some(st) = test_cmd_2(token_list) {
        Some(st)
    } else if let Some(st) = test_cmd_3(token_list) {
        Some(st)
    } else {
        None
    };
}
//	<simple command> '<' <filename>
fn test_cmd_1<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    None
}
//	<simple command> '>' <filename>
fn test_cmd_2<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    None
}
//	<simple command>
fn test_cmd_3<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    None
}

// test simple cmd production
fn test_simplecmd<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    None
}

// test tokenlist production
fn test_tokenlist<'a>(token_list: &[Token]) -> Option<SyntaxTree<'a>> {
    None
}

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser { ast: None }
    }

    pub fn parse(&mut self, token_list: VecDeque<Token>) -> Result<(), String> {
        let syntree = test_cmdline(token_list.as_slices().0);
        for i in token_list {
            println!("{:?}", i);
        }
        Ok(())
    }
}
