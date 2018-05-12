use parser::*;
use std::process::Command;

pub fn interpret_simplecmd_expr(expr: SimpleCmdExpr) -> Command {
    match expr {
        SimpleCmdExpr::Exe(exepath) => Command::new(exepath),
        SimpleCmdExpr::ExeWithArg(exepath, args) => {
            let mut cmd = Command::new(exepath);
            cmd.args(args);
            cmd
        }
    }
}

pub fn interpret_cmd_expr(expr: CommandExpr) {
    match expr {
        CommandExpr::Type1(simplecmd_expr) => {
            let output = interpret_simplecmd_expr(*simplecmd_expr).output();
            if output.is_ok() {
                println!("stdout: {}", String::from_utf8_lossy(&output.unwrap().stdout))
            }
        }
        CommandExpr::Type2(simplecmd_expr, op, filename) => {}
    }
}

pub fn interpret_job_expr(expr: JobExpr) {
    match expr {
        JobExpr::Type1(cmd_expr) => {
            interpret_cmd_expr(*cmd_expr);
        }
        JobExpr::Type2(cmd_expr, JobOp::Pipe, job_expr) => {}
    }
}

pub fn interpret_cmdline_expr(expr: CommandLineExpr) {
    match expr {
        CommandLineExpr::Type1(job_expr) => {
            interpret_job_expr(*job_expr);
        }
        CommandLineExpr::Type2(job_expr, op) => {}
        CommandLineExpr::Type3(job_expr, op, cmdline_expr) => {}
    }
}

pub fn interpret(expr: CommandLineExpr) {
    interpret_cmdline_expr(expr);
}
