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
            self.next();
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
                if self.match_next_char('=') {
                    self.add_token(token::TokenValue::NOT_EQUAL);
                } else {
                    self.add_token(token::TokenValue::NOT);
                }
            },
            '=' => {
                if self.match_next_char('=') {
                    self.add_token(token::TokenValue::EQUAL_EQUAL);
                } else {
                    self.add_token(token::TokenValue::EQUAL);
                }
            },
            '>' => {
                if self.match_next_char('=') {
                    self.add_token(token::TokenValue::GREATER_EQUAL);
                } else {
                    self.add_token(token::TokenValue::GREATER);
                }
            },
            '<' => {
                if self.match_next_char('=') {
                    self.add_token(token::TokenValue::LESS_EQUAL);
                } else {
                    self.add_token(token::TokenValue::LESS);
                }
            },
            '"' => self.get_string_contents(),
            '\n' => self.line += 1,
            '\t' | '\r' | ' ' | _ => {}
        };
    }

    fn is_at_end(&self) -> bool { 
        return self.curr as usize >= self.source.chars().count();
    }

    fn next(&mut self) {
        self.curr += 1;
    }

    fn add_token(&mut self, token: token::TokenValue) {
        self.tokens.push(token::Token::new(token, self.line));
    }

    fn match_next_char(&mut self, next: char) -> bool {
        if (self.source.as_bytes()[(self.curr + 1) as usize] as char) == next {
            self.next();
            return true;
        }
        return false;
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        return self.source.as_bytes()[(self.curr + 1) as usize] as char;
    }

    fn get_string_contents(&mut self) {
        let mut s: String = "".to_owned();
        
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            s.push(self.source.as_bytes()[(self.curr + 1) as usize] as char);
            self.next();
        }

        if(self.is_at_end()) {
            // Throw error
        }

        // Consume closing quote
        self.next();
    }
}