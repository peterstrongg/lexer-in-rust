pub enum TokenValue {
        // Single char tokens
        LEFTPAREN, RIGHTPAREN, LEFTCURLY, RIGHTCURLY,
        DOT, COMMA, MINUS, PLUS, STAR, SEMICOLON,
    
        // Literals
        NUMBER, STRING, IDENTIFIER,
    
        // Keywords
        AND, OR, IF, ELSE, WHILE, LET, CONST, TRUE, 
        FALSE, RETURN,

        // Empty Token
        NAN
}

pub struct Token {
    token: TokenValue,
    literal: String,
    line: i32
}

impl Token {
    pub fn new(token: TokenValue, literal: String, line: i32) -> Token {
        Token { token, literal, line }
    }
}