#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Integer,
    Add,
    Subtract,
    Multiply,
    Divide,
    EOF,
    Empty,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

impl Token {
    pub fn eof() -> Token {
        Token {
            kind: TokenType::EOF,
            value: "".to_string(),
        }
    }
}
