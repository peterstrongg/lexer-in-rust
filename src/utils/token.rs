pub enum TokenValue {
        // Single char tokens
        LEFTPAREN, RIGHTPAREN, LEFTCURLY, RIGHTCURLY,
        DOT, COMMA, MINUS, PLUS, STAR, SEMICOLON,
    
        // Literals
        NUMBER, STRING, IDENTIFIER,
    
        // Keywords
        AND, OR, IF, ELSE, WHILE, LET, CONST, TRUE, 
        FALSE, RETURN,

        // End of file
        EOF,
}

pub struct Token {
    token: TokenValue,
    line: i32
}

impl Token {
    pub fn new(token: TokenValue, line: i32) -> Token {
        Token { token, line }
    }
}