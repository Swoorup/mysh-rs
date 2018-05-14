use builtin::*;
use nix;
use nix::unistd::Pid;
use parser::*;
use std::fs::File;
use std::io::Result;
use std::io::{Error, ErrorKind};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

pub fn interpret_simplecmd_expr(expr: &SimpleCmdExpr) -> Command {
    match expr {
        SimpleCmdExpr::Exe(exepath) => {
            let mut cmd = Command::new(exepath);
            cmd.before_exec(|| {
                disable_shell_signal_handlers();
                Ok(())
            });
            cmd
        }
        SimpleCmdExpr::ExeWithArg(exepath, args) => {
            let mut cmd = Command::new(exepath);
            cmd.before_exec(|| {
                disable_shell_signal_handlers();
                Ok(())
            });
            cmd.args(args);
            cmd
        }
    }
}

pub fn interpret_cmd_expr(expr: &CommandExpr) -> Result<Command> {
    match expr {
        CommandExpr::Type1(box simplecmd_expr) => Ok(interpret_simplecmd_expr(simplecmd_expr)),
        CommandExpr::Type2(box simplecmd_expr, op, filename) => {
            let mut cmd = interpret_simplecmd_expr(simplecmd_expr);
            match op {
                CommandOp::RedirectIn => {
                    let f = File::open(filename)?;
                    cmd.stdin(Stdio::from(f));
                }
                CommandOp::RedirectOut => {
                    let f = File::create(filename)?;
                    cmd.stdout(Stdio::from(f));
                }
            }
            Ok(cmd)
        }
    }
}

pub fn interpret_job_expr(expr: &JobExpr) -> Result<Vec<u32>> {
    let mut stdio = Stdio::inherit();
    let mut vec: Vec<u32> = Vec::new();

    let mut inner_job_expr = expr;
    loop {
        match inner_job_expr {
            JobExpr::Type1(box lhs_cmd_expr) => {
                let child = interpret_cmd_expr(lhs_cmd_expr)?.stdin(stdio).spawn()?;
                vec.push(child.id());
                return Ok(vec);
            }
            JobExpr::Type2(box lhs_cmd_expr, JobOp::Pipe, box rhs_job_expr) => {
                let child = interpret_cmd_expr(lhs_cmd_expr)?
                    .stdin(stdio)
                    .stdout(Stdio::piped())
                    .spawn()?;
                vec.push(child.id());
                stdio = Stdio::from(child.stdout.unwrap());

                inner_job_expr = rhs_job_expr;
            }
        };
    }
}

pub fn interpret_cmdline_expr(expr: &CommandLineExpr) -> Result<()> {
    let wait_job = |job: Vec<u32>| -> Result<()> {
        let mut result: Result<()> = Ok(());
        job.iter().for_each(|id| {
            let pid = Pid::from_raw(*id as i32);
            if let Err(_) = nix::sys::wait::waitpid(pid, None) {
                result = Err(Error::from(ErrorKind::Other));
            }
        });
        result
    };

    match expr {
        CommandLineExpr::Type1(box job_expr)
        | CommandLineExpr::Type2(box job_expr, CommandLineOp::Sequence) => {
            interpret_job_expr(job_expr).and_then(|v| wait_job(v))
        }
        CommandLineExpr::Type2(box job_expr, CommandLineOp::Background) => {
            interpret_job_expr(job_expr).map(|_| ())
        }
        CommandLineExpr::Type3(box job_expr, op, box cmdline_expr) => {
            match op {
                CommandLineOp::Background => {
                    if let Err(e) = interpret_job_expr(job_expr) {
                        return Err(e);
                    }
                }
                CommandLineOp::Sequence => {
                    if let Err(e) = interpret_job_expr(job_expr).and_then(|v| wait_job(v)) {
                        return Err(e);
                    }
                }
            }

            interpret_cmdline_expr(cmdline_expr)
        }
    }
}

pub fn interpret(expr: CommandLineExpr) -> Result<()> {
    interpret_cmdline_expr(&expr)
}
