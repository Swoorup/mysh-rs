use LexicalAnalyzer;

struct SyntaxTree {
    left: Option<Box<SyntaxTree>>,
    right: Option<Box<SyntaxTree>>,
}

pub struct Parser {
    ast: Option<SyntaxTree>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            ast: None,
        }
    }
}