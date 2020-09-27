#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    //core keywords
    Struct,
    Function,
    Method,
    Trait,
    Closure,
    Impl,
    Enum,
    Return,
    This,


    //control flow and logic
    If,
    Else,
    Match,
    And,        // &&
    Or,         // ||
    True,
    False,
    While,
    For,

    //error handling
    Try,
    Catch,
    Finally,

    //types and identifiers
    Ident,
    Strng,
    Int,
    Float,
    Nil,

    //operators
    Assgn,      // =
    Plus,       // +
    Minus,      // -
    Times,      //\*
    Divide,     // /
    Eq,         // ==
    Neq,        //\!=
    GEq,        // >=
    LEq,        // <=

    //delimiters
    StmtEnd,    // ;
    LeftCBkt,   // (
    RightCBkt,  // )
    LeftSqBkt,  // [
    RightSqBkt, // ]
    OpenBlock,  // {
    CloseBlock, // }

    //tokens that throw an error
    Unknown,    // Exits with an error if encountered
}

impl From<TokenType> for String {
    fn from(t: TokenType) -> String {
        String::new() //TODO: Implement this
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    tokentype: TokenType,
    lexeme: String,
    line: u64,
    column: u64,
}

impl Token {
    pub fn to_string(&self) -> String {
        String::from(self.tokentype.clone()) //might need to clone here in the future
    } 
}

#[derive(Clone, Debug)]
pub struct Lexer {
    current_char: u64,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            current_char: 0,
            tokens: Vec::new(),
        }
    }

    pub fn scan(&self, input: String) -> Vec<Token> {
        for c in input.chars() {

        }
        Vec::new()
    }
}
