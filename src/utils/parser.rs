use super::token;
use super::expression;
use utils::expression::Binary;

pub struct Parser {
    tokens: Vec<token::Token>,
    current: i32
}

impl Default for Parser {
    fn default() -> Parser {
        Parser {
            tokens: Vec::new(),
            current: 0
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<token::Token>) -> Parser {
        Parser { tokens, ..Default::default() }
    }

    fn expression(&mut self) -> expression::Expression {
        return self.equality();
    }

    fn equality(&mut self) -> expression::Expression {
        let mut expr = self.comparison();
        let tokens_to_check: [token::TokenValue; 2] = [
            token::TokenValue::NOT_EQUAL,
            token::TokenValue::EQUAL_EQUAL
        ]; 
        
        for tok in tokens_to_check.iter() {
            if self.match_tokens(*tok) {
                let operator: token::Token = self.previous();
                let right: expression::Expression = self.comparison();
                expr = expression::Expression::binary(expr, operator, right); 
            }
        }

        return expr;
    }

    fn comparison(&mut self) -> expression::Expression {
        let mut expr: expression::Expression = self.term();

        let tokens_to_check: [token::TokenValue; 4] = [
            token::TokenValue::GREATER,
            token::TokenValue::GREATER_EQUAL, 
            token::TokenValue::LESS, 
            token::TokenValue::LESS_EQUAL
        ];

        for tok in tokens_to_check.iter() {
            if self.match_tokens(*tok) {
                let operator: token::Token = self.previous();
                let right: expression::Expression = self.comparison();
                expr = expression::Expression::binary(expr, operator, right); 
            }
        }

        return expr;
    }

    fn term(&mut self) -> expression::Expression {
        return self.factor();
    }

    fn factor(&mut self) -> expression::Expression {
        return self.unary();
    }

    fn unary(&mut self) -> expression::Expression {
        return self.primary();
    }

    fn primary(&mut self) -> expression::Expression {
        return expression::Expression::new();
    } 

    fn match_tokens(&mut self, token: token::TokenValue) -> bool {
        if self.check(token) {
            self.next();
            return true;
        }
        return false;
    }

    fn check(&self, tok: token::TokenValue) -> bool {
        if self.is_at_end() { 
            return false;
        }
        return self.peek().token == tok;
    }

    fn peek(&self) -> token::Token {
        return self.tokens[self.current as usize].clone();
    }

    fn next(&mut self) -> token::Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn previous(&self) -> token::Token {
        return self.tokens[(self.current - 1) as usize].clone();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token == token::TokenValue::EOF;
    }
}