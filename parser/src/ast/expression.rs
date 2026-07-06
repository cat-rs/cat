use crate::ast::primitives::Path;

#[derive(Debug, Clone)]
pub enum Expression {
    Primary(Primary),
    Ref(Box<Expression>),
    Call {
        target: Box<Expression>,
        args: Vec<Expression>,
    },
    BinaryOP {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Primary {
    Path(Path),
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
}
