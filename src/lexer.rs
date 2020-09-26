pub enum Token {
    Struct,
    Function,
    Trait,
    Closure,
    Impl,
    Enum,
    If,
    Else,
    Nil,
    Int,
    Float,
    Try,
    Catch,
    StmtEnd,    // ;
    OpenCBkt,   // (
    CloseCBkt,  // )
    OpenSqBkt,  // [
    CloseSqBkt, // ]
    OpenBlock,  // {
    CloseBlock, // }
    Unknown,    // Exits with an error if encountered
}

pub struct Lexer {
    current: u64,
}

impl Lexer {
    pub fn new() {
        Lexer {
            current: 0,
        }
    }

    pub fn scan(&self, input: String) { //use String or &str?

    }
}

pub struct Parser {

}