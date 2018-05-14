/**
 *
	<command line>	::=  	<job>
						|	<job> '&'
						| 	<job> '&' <command line>
						|	<job> ';'
						|	<job> ';' <command line>

	<job>			::=		<command>
						|	< job > '|' < command >

	<command>		::=		<simple command>
						|	<simple command> '<' <filename>
						|	<simple command> '>' <filename>

	<simple command>::=		<pathname>
						|	<simple command>  <token>
 *
 *
 *
**/
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CommandLineOp {
    Background,
    Sequence,
}

#[derive(Debug, PartialEq)]
pub enum JobOp {
    Pipe,
}

#[derive(Debug, PartialEq)]
pub enum CommandOp {
    RedirectIn,
    RedirectOut,
}

#[derive(Debug, PartialEq)]
pub enum CommandLineExpr {
    Type1(Box<JobExpr>),
    Type2(Box<JobExpr>, CommandLineOp),
    Type3(Box<JobExpr>, CommandLineOp, Box<CommandLineExpr>),
}

#[derive(Debug, PartialEq)]
pub enum JobExpr {
    Type1(Box<CommandExpr>),
    Type2(Box<CommandExpr>, JobOp, Box<JobExpr>),
}

#[derive(Debug, PartialEq)]
pub enum CommandExpr {
    Type1(Box<SimpleCmdExpr>),
    Type2(Box<SimpleCmdExpr>, CommandOp, String),
}

#[derive(PartialEq)]
pub enum SimpleCmdExpr {
    Exe(String),
    ExeWithArg(String, Vec<String>),
}

impl fmt::Debug for SimpleCmdExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            SimpleCmdExpr::Exe(exepath) => write!(f, "{:?}", exepath),
            SimpleCmdExpr::ExeWithArg(exepath, args) => write!(f, "{:?} {:?}", exepath, args),
        }
    }
}
