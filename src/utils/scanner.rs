use super::token;

pub struct Scanner {
    source: String,
    tokens: Vec<super::token::Token>,
    start: i32,
    curr: i32,
    line: i32
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: "".to_owned(),
            tokens: Vec::new(),
            start: 0,
            curr: 0,
            line: 1
        }
    }
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source, ..Default::default() }
    }

    pub fn scan_for_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.curr;
            self.scan_token();
        }

        self.add_token(token::TokenValue::EOF);
    }

    fn scan_token(&mut self) {
        let ch: char = self.source.as_bytes()[self.curr as usize] as char;
        match ch {
            // Single char tokens
            '(' => self.add_token(token::TokenValue::LEFTPAREN),
            ')' => self.add_token(token::TokenValue::RIGHTPAREN),
            '{' => self.add_token(token::TokenValue::LEFTCURLY),
            '}' => self.add_token(token::TokenValue::RIGHTCURLY),
            '.' => self.add_token(token::TokenValue::DOT),
            ',' => self.add_token(token::TokenValue::COMMA),
            '-' => self.add_token(token::TokenValue::MINUS),
            '+' => self.add_token(token::TokenValue::PLUS),
            '*' => self.add_token(token::TokenValue::STAR),
            ';' => self.add_token(token::TokenValue::SEMICOLON),

            // One or two char tokens
            '!' => {
                if self.next_byte() == '=' {
                    self.add_token(token::TokenValue::NOT_EQUAL);
                } else {
                    self.add_token(token::TokenValue::NOT);
                }
            },
            '=' => {
                if self.next_byte() == '=' {
                    self.add_token(token::TokenValue::EQUAL_EQUAL);
                } else {
                    self.add_token(token::TokenValue::EQUAL);
                }
            },
            '>' => {
                if self.next_byte() == '=' {
                    self.add_token(token::TokenValue::GREATER_EQUAL);
                } else {
                    self.add_token(token::TokenValue::GREATER);
                }
            },
            '<' => {
                if self.next_byte() == '=' {
                    self.add_token(token::TokenValue::LESS_EQUAL);
                } else {
                    self.add_token(token::TokenValue::LESS);
                }
            },
            '\n' => self.line += 1,
            '\t' | '\r' | ' ' | _ => {}
        };
        self.curr += 1;
    }

    fn is_at_end(&self) -> bool { 
        return self.curr as usize >= self.source.chars().count();
    }

    fn add_token(&mut self, token: token::TokenValue) {
        self.tokens.push(token::Token::new(token, self.line));
    }

    fn next_byte(&self) -> char {
        return self.source.as_bytes()[(self.curr + 1) as usize] as char;
    }
}