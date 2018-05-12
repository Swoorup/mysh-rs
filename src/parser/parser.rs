use std::iter::Iterator;

use parser::*;

pub struct Parser<T> {
    tok_iter: T,
}

impl<'a, T> Parser<T>
where
    T: TokenIter<'a>,
{
    pub fn new(toker_iter: T) -> Parser<T> {
        Parser {
            tok_iter: toker_iter,
        }
    }

    pub fn create_commandline_expr(&mut self) -> Option<Box<CommandLineExpr>> {
        let job_expr = self.create_job_expr()?;

        let mut cloned_iter = self.tok_iter.clone();
        let tok = cloned_iter.next();
        if tok.is_none() || !tok.unwrap().is_symbol()
            || !(tok.unwrap().symbol().unwrap() == "&" || tok.unwrap().symbol().unwrap() == ";")
        {
            return Some(Box::new(CommandLineExpr::Type1(job_expr)));
        }

        let symbol = tok.unwrap().symbol().unwrap();
        let mut cmd_line_op = CommandLineOp::Background;
        if symbol == ";" {
            cmd_line_op = CommandLineOp::Sequence;
        }

        self.tok_iter.next();
        let cloned_iter = self.tok_iter.clone();
        let next_cmdline_expr = self.create_commandline_expr();
        if next_cmdline_expr.is_none() {
            self.tok_iter = cloned_iter.clone();
            return Some(Box::new(CommandLineExpr::Type2(job_expr, cmd_line_op)));
        }

        return Some(Box::new(CommandLineExpr::Type3(
            job_expr,
            cmd_line_op,
            next_cmdline_expr.unwrap(),
        )));
    }

    pub fn create_job_expr(&mut self) -> Option<Box<JobExpr>> {
        let command_expr = self.create_command_expr()?;

        let mut cloned_iter = self.tok_iter.clone();
        let tok = cloned_iter.next();
        if tok.is_none() || !tok.unwrap().is_symbol() || tok.unwrap().symbol().unwrap() != "|" {
            return Some(Box::new(JobExpr::Type1(command_expr)));
        }

        self.tok_iter.next();
        let next_job_expr = self.create_job_expr();
        if next_job_expr.is_none() {
            self.tok_iter = cloned_iter.clone();
            return None;
        }

        return Some(Box::new(JobExpr::Type2(
            command_expr,
            JobOp::Pipe,
            next_job_expr.unwrap(),
        )));
    }

    pub fn create_command_expr(&mut self) -> Option<Box<CommandExpr>> {
        let simplecmd_expr = self.create_simplecmd_expr()?;

        let mut cloned_iter = self.tok_iter.clone().peekable();
        let tok = cloned_iter.next();
        if tok.is_none() || !tok.unwrap().is_symbol()
            || !(tok.unwrap().symbol().unwrap() == "<" || tok.unwrap().symbol().unwrap() == ">")
        {
            return Some(Box::new(CommandExpr::Type1(simplecmd_expr)));
        }

        let symbol = tok.unwrap().symbol().unwrap();
        let mut redir = CommandOp::RedirectIn;
        if symbol == ">" {
            redir = CommandOp::RedirectOut;
        }

        let tok = cloned_iter.next();
        if tok.is_none() || !tok.unwrap().is_varstring() {
            return Some(Box::new(CommandExpr::Type1(simplecmd_expr))); // error unexpected token
        }

        for _ in 0..2 {
            self.tok_iter.next();
        }
        return Some(Box::new(CommandExpr::Type2(
            simplecmd_expr,
            redir,
            tok.unwrap().varstring().unwrap(),
        )));
    }

    pub fn create_simplecmd_expr(&mut self) -> Option<Box<SimpleCmdExpr>> {
        let mut cloned_iter = self.tok_iter.clone().peekable();

        let tok = cloned_iter.next()?;
        if let Token::VarString(filepath) = tok {
            let exepath = filepath.to_string();

            if cloned_iter.peek().is_none() || !cloned_iter.peek().unwrap().is_varstring() {
                self.tok_iter.next();
                return Some(Box::new(SimpleCmdExpr::Exe(exepath)));
            }

            let args: Vec<_> = cloned_iter
                .by_ref()
                .take_while(|t| t.is_varstring())
                .map(|t| t.varstring().unwrap().clone())
                .collect();

            for _ in 0..=args.len() {
                self.tok_iter.next();
            }

            return Some(Box::new(SimpleCmdExpr::ExeWithArg(exepath, args)));
        } else {
            return None;
        }
    }

    pub fn debug_print_next_token(&mut self) {
        println!("Next remaining: {:?}", self.tok_iter.next());
    }

    pub fn parse(&mut self) -> Result<Option<Box<CommandLineExpr>>, String> {
        let syntree = self.create_commandline_expr();
        let remaining_tok = self.tok_iter.next();
        if remaining_tok.is_some() {
            return Err(format!("Unexpected token: {:?}", remaining_tok.unwrap()));
        }
        Ok(syntree)
    }

}
