use super::token;
use super::expression;

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

        

        return expr;
    }

    fn comparison(&self) -> expression::Expression {
        return self.term();
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
}