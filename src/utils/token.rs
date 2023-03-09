enum TokenValue {
        // Single char tokens
        LEFTPAREN, RIGHTPAREN, LEFTCURLY, RIGHTCURLY,
        DOT, COMMA, MINUS, PLUS, STAR, SEMICOLON, SLASH,
    
        // Literals
        NUMBER, STRING, IDENTIFIER,
    
        // Keywords
        AND, OR, IF, ELSE, WHILE, LET, CONST, TRUE, 
        FALSE, RETURN,
}

pub struct Token {
    token: TokenValue,
    lexeme: String,
    line: i32
}

impl Token {
    fn new(token: TokenValue, lexeme: String, line: i32) -> Token {
        Token { token, lexeme, line }
    }
}