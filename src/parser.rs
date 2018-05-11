use lexer::Token;
use lexer::TokenIter;

#[derive(Debug)]
struct SyntaxTree<'a> {
    token: &'a Token<'a>,
    left: Option<Box<SyntaxTree<'a>>>,
    right: Option<Box<SyntaxTree<'a>>>,
}

impl<'a> SyntaxTree<'a> {
    fn new(
        token: &'a Token,
        left_node: Option<Box<SyntaxTree<'a>>>,
        right_node: Option<Box<SyntaxTree<'a>>>,
    ) -> Box<SyntaxTree<'a>> {
        Box::new(SyntaxTree {
            left: left_node,
            right: right_node,
            token: token,
        })
    }

    fn set_left(mut self, token: &'a Token) -> SyntaxTree<'a> {
        self.left = Some(Box::new(SyntaxTree {
            left: None,
            right: None,
            token: token,
        }));
        self
    }

    fn set_right(mut self, token: &'a Token) -> SyntaxTree<'a> {
        self.right = Some(Box::new(SyntaxTree {
            left: None,
            right: None,
            token: token,
        }));
        self
    }
}

#[test]
fn test_syntax_tree() {
    let mut syntaxtree = SyntaxTree::new(&Token::WhiteSpace, None, None)
        .set_left(&Token::WhiteSpace)
        .set_right(&Token::WhiteSpace);
    println!("{:#?}", syntaxtree);
}

pub struct Parser<'a, T> {
    ast: Option<Box<SyntaxTree<'a>>>,
    tok_iter: T,
}

impl<'a, T> Parser<'a, T>
where
    T: TokenIter<'a>,
{
    pub fn new(toker_iter: T) -> Parser<'a, T> {
        Parser {
            ast: None,
            tok_iter: toker_iter,
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        let syntree = self.test_cmdline();
        if syntree.is_some() {
            println!("{:#?}", syntree.unwrap());
        }

        if self.tok_iter.next().is_some() {
            println!("Unexpected token");
        }
        Ok(())
    }

    //	test all command line production orderwise
    fn test_cmdline(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        let funcs = [
            Self::test_cmdline_1,
            Self::test_cmdline_2,
            Self::test_cmdline_3,
            Self::test_cmdline_4,
            Self::test_cmdline_5,
        ];

        let cloned_iter = self.tok_iter.clone(); // to reset if test fails
        for f in funcs.iter() {
            self.tok_iter = cloned_iter.clone();
            if let Some(st) = f(self) {
                return Some(st);
            }
        }

        self.tok_iter = cloned_iter;
        None
    }

    //	<job> ';' <command line>
    fn test_cmdline_1(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        let job_node = Some(self.test_job()?);

        let term = self.tok_iter.next()?;
        if term != &Token::Symbol(";") {
            return None;
        }

        let cmd_line_node = Some(self.test_cmdline()?);

        Some(SyntaxTree::new(term, job_node, cmd_line_node))
    }
    //	<job> ';'
    fn test_cmdline_2(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        let job_node = Some(self.test_job()?);

        let term = self.tok_iter.next()?;
        if term != &Token::Symbol(";") {
            return None;
        }

        Some(SyntaxTree::new(term, job_node, None))
    }
    //	<job> '&' <command line>
    fn test_cmdline_3(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        let job_node = Some(self.test_job()?);
        let term = self.tok_iter.next()?;
        if term != &Token::Symbol("&") {
            return None;
        }

        let cmd_line_node = Some(self.test_cmdline()?);

        Some(SyntaxTree::new(term, job_node, cmd_line_node))
    }
    //	<job> '&'
    fn test_cmdline_4(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        let job_node = Some(self.test_job()?);
        let term = self.tok_iter.next()?;
        if term != &Token::Symbol("&") {
            return None;
        }

        Some(SyntaxTree::new(term, job_node, None))
    }
    //	<job>
    fn test_cmdline_5(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        self.test_job()
    }

    // test all job production in order
    fn test_job(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        let funcs = [Self::test_job_1, Self::test_job_2];

        let cloned_iter = self.tok_iter.clone(); // to reset if test fails
        for f in funcs.iter() {
            self.tok_iter = cloned_iter.clone();
            if let Some(st) = f(self) {
                return Some(st);
            }
        }

        self.tok_iter = cloned_iter;
        None
    }
    // <command> '|' <job>
    fn test_job_1(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        let cmd_node = Some(self.test_cmd()?);
        let term = self.tok_iter.next()?;
        if term != &Token::Symbol("|") {
            return None;
        }

        let job_node = Some(self.test_job()?);

        Some(SyntaxTree::new(term, cmd_node, job_node))
    }

    // <command>
    fn test_job_2(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        self.test_cmd()
    }

    // test all command production orderwise
    fn test_cmd(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        let funcs = [Self::test_cmd_1, Self::test_cmd_2, Self::test_cmd_3];

        let cloned_iter = self.tok_iter.clone(); // to reset if test fails
        for f in funcs.iter() {
            self.tok_iter = cloned_iter.clone();
            if let Some(st) = f(self) {
                return Some(st);
            }
        }

        self.tok_iter = cloned_iter;
        None
    }

    //	<simple command> '<' <filename>
    fn test_cmd_1(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        let simplecmd_node = Some(self.test_simplecmd()?);
        let term = self.tok_iter.next()?;
        if term != &Token::Symbol("<") {
            return None;
        }

        let term_filename = self.tok_iter.next()?;
        if let Token::VarString(_) = term_filename {
            Some(SyntaxTree::new(
                term,
                simplecmd_node,
                Some(SyntaxTree::new(term_filename, None, None)),
            ))
        } else {
            None
        }
    }
    //	<simple command> '>' <filename>
    fn test_cmd_2(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        let simplecmd_node = Some(self.test_simplecmd()?);
        let term = self.tok_iter.next()?;
        if term != &Token::Symbol(">") {
            return None;
        }

        let term_filename = self.tok_iter.next()?;
        match term_filename {
            Token::VarString(_) => Some(SyntaxTree::new(
                term,
                simplecmd_node,
                Some(SyntaxTree::new(term_filename, None, None)),
            )),
            _ => None,
        }
        // if let Token::VarString(_) = term_filename {
        //     Some(SyntaxTree::new(
        //         term,
        //         simplecmd_node,
        //         Some(SyntaxTree::new(term_filename, None, None)),
        //     ))
        // } else {
        //     None
        // }
    }

    //	<simple command>
    fn test_cmd_3(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        self.test_simplecmd()
    }

    // test simple cmd production
    // <pathname> <token list>
    fn test_simplecmd(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        let cloned_iter = self.tok_iter.clone();

        let term_pathname = self.tok_iter.next()?;
        if let Token::VarString(_) = term_pathname {
            let tokenlist_node = self.test_tokenlist();
            // don't check since its a valid grammer

            Some(SyntaxTree::new(term_pathname, None, tokenlist_node))
        } else {
            self.tok_iter = cloned_iter;
            None
        }
    }

    // test tokenlist production
    // <token> <token list>
    fn test_tokenlist(&mut self) -> Option<Box<SyntaxTree<'a>>> {
        let cloned_iter = self.tok_iter.clone();

        let token = self.tok_iter.next()?;
        if let Token::VarString(_) = token {
            let tokenlist_node = self.test_tokenlist();
            // don't check since its a valid grammer

            Some(SyntaxTree::new(token, None, tokenlist_node))
        } else {
            self.tok_iter = cloned_iter;
            None
        }
    }
}
