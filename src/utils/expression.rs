use super::token;

pub struct Expression {
    operator: token::Token,
    left: Box<Expression>,
    right: Box<Expression>
}

impl Default for Expression {
    fn default() -> Expression {
        Expression {
            operator: Default::default(),
            left: Default::default(),
            right: Default::default(),
        }
    }
}

impl Expression {
    pub fn new() -> Expression {
        Expression { ..Default::default() }
    }
}

pub trait Binary {
    fn binary(&mut self, l:Expression, op:token::Token, r:Expression);
}

pub trait Unary {
    fn unary(&mut self, op:token::Token, r:Expression);
}

impl Binary for Expression {
    fn binary(&mut self, l: Expression, op: token::Token, r: Expression) {
        self.operator = op;
        *self.left = l;
        *self.right = r;
    }
}

impl Unary for Expression {
    fn unary(&mut self, op: token::Token, r: Expression) {
        self.operator = op;
        *self.right = r
    }
} 