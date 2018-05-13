use parser::*;
use std::fs::File;
use std::io::Result;
use std::process::{Child, Command, Stdio};

pub fn interpret_simplecmd_expr(expr: &SimpleCmdExpr) -> Command {
    match expr {
        SimpleCmdExpr::Exe(exepath) => Command::new(exepath),
        SimpleCmdExpr::ExeWithArg(exepath, args) => {
            let mut cmd = Command::new(exepath);
            cmd.args(args);
            cmd
        }
    }
}

pub fn interpret_cmd_expr(expr: &CommandExpr) -> Command {
    match expr {
        CommandExpr::Type1(box simplecmd_expr) => interpret_simplecmd_expr(simplecmd_expr),
        CommandExpr::Type2(box simplecmd_expr, op, filename) => {
            let mut cmd = interpret_simplecmd_expr(simplecmd_expr);
            match op {
                CommandOp::RedirectIn => {
                    let f = File::open(filename).unwrap();
                    cmd.stdin(Stdio::from(f));
                }
                CommandOp::RedirectOut => {
                    let f = File::create(filename).unwrap();
                    cmd.stdout(Stdio::from(f));
                }
            }
            cmd
        }
    }
}

pub fn interpret_job_expr(expr: &JobExpr) -> Result<Vec<Child>> {
    let stdio = Stdio::inherit();
    match expr {
        JobExpr::Type1(box cmd_expr) => {
            let mut cmd = interpret_cmd_expr(cmd_expr);
            Ok(vec![cmd.spawn()?])
        }
        JobExpr::Type2(box cmd_expr, JobOp::Pipe, box job_expr) => {
            let mut cmd = interpret_cmd_expr(cmd_expr);
            cmd.stdout(Stdio::piped());
            let child = cmd.spawn()?;
            let mut output = child.stdout.unwrap();

            let mut inner_job_expr = job_expr;
            loop {
                match inner_job_expr {
                    JobExpr::Type1(box lhs_cmd_expr) => {
                        let mut cmd = interpret_cmd_expr(lhs_cmd_expr);
                        cmd.stdin(output);
                        return Ok(vec![cmd.spawn()?]);
                    }
                    JobExpr::Type2(box lhs_cmd_expr, JobOp::Pipe, box rhs_job_expr) => {
                        let mut cmd = interpret_cmd_expr(lhs_cmd_expr);
                        cmd.stdin(output).stdout(Stdio::piped());
                        inner_job_expr = rhs_job_expr;
                        output = cmd.spawn()?.stdout.unwrap();
                    }
                };
            }
        }
    }
}

pub fn interpret_cmdline_expr(expr: &CommandLineExpr) {
    match expr {
        CommandLineExpr::Type1(box job_expr) => {
            interpret_job_expr(job_expr);
        }
        CommandLineExpr::Type2(box job_expr, op) => match op {
            CommandLineOp::Background => {
                interpret_job_expr(job_expr);
            }
            CommandLineOp::Sequence => {
                interpret_job_expr(job_expr);
            }
        },
        CommandLineExpr::Type3(box job_expr, op, box cmdline_expr) => {
            match op {
                CommandLineOp::Background => {
                    interpret_job_expr(job_expr);
                }
                CommandLineOp::Sequence => {
                    interpret_job_expr(job_expr);
                }
            }

            interpret_cmdline_expr(cmdline_expr);
        }
    }
}

pub fn interpret(expr: CommandLineExpr) {
    interpret_cmdline_expr(&expr);
}
