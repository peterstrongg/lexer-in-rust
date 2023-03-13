use std::collections::HashMap;
use std::clone::Clone;
use super::token;

pub struct Scanner {
    source: String,
    tokens: Vec<super::token::Token>,
    start: i32,
    curr: i32,
    line: i32,
    test_map: HashMap<String, token::TokenValue>
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: "".to_owned(),
            tokens: Vec::new(),
            start: 0,
            curr: 0,
            line: 1,
            test_map: HashMap::from([
                ("and".to_owned(), token::TokenValue::AND),
                ("or".to_owned(), token::TokenValue::OR),
                ("if".to_owned(), token::TokenValue::IF),
                ("else".to_owned(), token::TokenValue::ELSE),
                ("while".to_owned(), token::TokenValue::WHILE),
                ("let".to_owned(), token::TokenValue::LET),
                ("const".to_owned(), token::TokenValue::CONST),
                ("true".to_owned(), token::TokenValue::TRUE),
                ("false".to_owned(), token::TokenValue::FALSE),
                ("return".to_owned(), token::TokenValue::RETURN),
            ])
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
        self.next();
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
            '"' => self.add_string_token(),
            '\n' => self.line += 1,
            '\t' | '\r' | ' ' => {},

            // Default case
            _ => {
                if ch.is_numeric() {
                    self.add_number_token();
                } else if ch.is_alphabetic() {
                    self.add_identifier_token();
                } else {
                    // Throw unexpected character error
                }
            }
        };
    }

    fn is_at_end(&self) -> bool { 
        return self.curr as usize >= self.source.chars().count();
    }

    fn next(&mut self) {
        self.curr += 1;
    }

    fn curr_char(&self) -> char {
        return self.source.as_bytes()[(self.curr - 1) as usize] as char;
    }

    fn add_token(&mut self, token: token::TokenValue) {
        self.tokens.push(token::Token::new(token, self.line));
    }

    fn match_next_char(&mut self, next: char) -> bool {
        if (self.source.as_bytes()[(self.curr) as usize] as char) == next {
            self.next();
            return true;
        }
        return false;
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        return self.source.as_bytes()[(self.curr) as usize] as char;
    }

    fn add_string_token(&mut self) {
        let mut s: String = "".to_owned();

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            s.push(self.peek() as char);
            self.next();
        }

        if self.is_at_end() {
            // Throw non terminated string error
            return;
        }

        // Consume closing quote
        self.next();

        // Add token with string literal
        let mut t = token::Token::new(token::TokenValue::STRING, self.line);
        t.set_str(s);
        self.tokens.push(t);
    }

    fn add_number_token(&mut self) {
        let mut num: String = "".to_owned();

        num.push(self.curr_char());
        while self.peek().is_numeric() {
            num.push(self.peek());
            self.next();
        }

        // Number is a float
        if self.peek() == '.' {
            num.push('.');
            self.next();
            if self.peek().is_numeric() {
                while self.peek().is_numeric() {
                    num.push(self.peek());
                    self.next();
                }
                let mut float_token = token::Token::new(token::TokenValue::FLOAT, self.line);
                match num.parse::<f32>() {
                    Ok(val) => {
                        float_token.set_flt(val); 
                        self.tokens.push(float_token);
                    },
                    Err(why) => println!("Error converting to float"),
                }
            } else {
                // Throw bad float error
            }
            return;
        }
        
        let mut int_token = token::Token::new(token::TokenValue::INTEGER, self.line);
        match num.parse::<i32>() {
            Ok(val) => {
                int_token.set_int(val);
                self.tokens.push(int_token);
            },
            Err(why) => println!("Error converting to int"),
        }
    }

    fn add_identifier_token(&mut self) {
        let mut s: String = "".to_owned();

        s.push(self.curr_char());
        while self.peek().is_alphabetic() {
            s.push(self.peek());
            self.next();
        }
        
        match self.test_map.get(&s) {
            Some(val) => self.add_token(*val),
            None => self.tokens.push(token::Token::new(token::TokenValue::IDENTIFIER, self.line))
        }
    }
}