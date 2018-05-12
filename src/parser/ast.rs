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

#[derive(Debug)]
pub enum CommandLineOp {
    Background,
    Sequence,
}

#[derive(Debug)]
pub enum JobOp {
    Pipe,
}

#[derive(Debug)]
pub enum CommandOp {
    RedirectIn,
    RedirectOut,
}

#[derive(Debug)]
pub enum CommandLineExpr {
    Type1(Box<JobExpr>),
    Type2(Box<JobExpr>, CommandLineOp),
    Type3(Box<JobExpr>, CommandLineOp, Box<CommandLineExpr>),
}

#[derive(Debug)]
pub enum JobExpr {
    Type1(Box<CommandExpr>),
    Type2(Box<CommandExpr>, JobOp, Box<JobExpr>),
}

#[derive(Debug)]
pub enum CommandExpr {
    Type1(Box<SimpleCmdExpr>),
    Type2(Box<SimpleCmdExpr>, CommandOp, String),
}

#[derive(Debug)]
pub enum SimpleCmdExpr {
    Exe(String),
    ExeWithArg(String, Vec<String>),
}