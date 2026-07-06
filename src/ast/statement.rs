use crate::ast::primitives::{Identifier, TypeExpr};

pub struct Block(Vec<Statement>);

pub struct FnParam {
    type_: TypeExpr,
    name: Identifier,
}

pub enum Statement {
    FunctionDeclaration {
        return_type: TypeExpr,
        name: Identifier,
        params: Vec<FnParam>,
        body: Option<Block>,
    },
}
