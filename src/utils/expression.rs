use super::token;

pub struct Expression {
    operator: Option<token::Token>,
    left: Option<Box<Expression>>,
    right: Option<Box<Expression>>
}

impl Default for Expression {
    fn default() -> Expression {
        Expression {
            operator: None,
            left: None,
            right: None,
        }
    }
}

impl Expression {
    pub fn new() -> Expression {
        Expression { ..Default::default() }
    }
}

pub trait Binary {
    fn binary(l:Expression, op:token::Token, r:Expression) -> Self;
}

pub trait Unary {
    fn unary(&mut self, op:token::Token, r:Expression);
}

impl Binary for Expression {
    fn binary(l: Expression, op: token::Token, r: Expression) -> Self {
        let mut e = Expression::new();
        e.operator = Some(op);
        e.left = Some(Box::new(l));
        e.right = Some(Box::new(r));
        return e;

        // self.operator = Some(op);
        // self.left = Some(Box::new(l));
        // self.right = Some(Box::new(r));
    }
}

impl Unary for Expression {
    fn unary(&mut self, op: token::Token, r: Expression) {
        self.operator = Some(op);
        self.right = Some(Box::new(r));
    }
} 