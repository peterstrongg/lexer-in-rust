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

    fn expression(&self) -> expression::Expression {
        return self.equality();
    }

    fn equality(&self) -> expression::Expression {
        let mut expr = self.comparison();

        self.match_tokens(
            &[token::TokenValue::NOT_EQUAL, token::TokenValue::EQUAL_EQUAL]
        );
        

        return expr;
    }

    fn comparison(&self) -> expression::Expression {
        let mut expr: expression::Expression = self.term();

        let right: expression::Expression = self.term();
        expr = expression::Expression::binary(expr, token::Token::new(token::TokenValue::EOF, 0), right);

        return expr;
    }

    fn term(&self) -> expression::Expression {
        return self.factor();
    }

    fn factor(&self) -> expression::Expression {
        return self.unary();
    }

    fn unary(&self) -> expression::Expression {
        return self.primary();
    }

    fn primary(&self) -> expression::Expression {
        return expression::Expression::new();
    } 

    fn match_tokens(&self, args: &[token::TokenValue]) {

    }

    fn check(&self) {

    }

    fn peek(&self) -> token::Token {
        return self.tokens[self.current as usize].clone();
    }

    fn next(&mut self) {
        self.current += 1;
    }
}