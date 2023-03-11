pub enum TokenValue {
        // Single char tokens
        LEFTPAREN, RIGHTPAREN, LEFTCURLY, RIGHTCURLY,
        DOT, COMMA, MINUS, PLUS, STAR, SEMICOLON,

        // One or two character tokens
        NOT, NOT_EQUAL, EQUAL, LESS_EQUAL, GREATER_EQUAL, 
        EQUAL_EQUAL, LESS, GREATER,

        // Literals
        NUMBER, STRING, IDENTIFIER,
    
        // Keywords
        AND, OR, IF, ELSE, WHILE, LET, CONST, TRUE, 
        FALSE, RETURN,

        // End of file
        EOF,

        NIL
}

pub struct Token {
    token: TokenValue,
    line: i32,
    literal_str: Option<String>,
    literal_int: Option<i32>,
    literal_flt: Option<f32>,
}

impl Default for Token {
    fn default() -> Token {
        Token {
            token: TokenValue::NIL,
            line: 0,
            literal_str: Default::default(),
            literal_int: Default::default(),
            literal_flt: Default::default(),
        }
    }
}

impl Token {
    pub fn new(token: TokenValue, line: i32) -> Token {
        Token { token, line, ..Default::default() }
    }

    pub fn set_str(&mut self, s: String) {
        self.literal_str = Some(s);
    }

    pub fn set_int(&mut self, i: i32) {
        self.literal_int = Some(i);
    }

    pub fn set_flt(&mut self, f: f32) {
        self.literal_flt = Some(f);
    }
}