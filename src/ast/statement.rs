use crate::ast::primitives::{Identifier, TypeExpr};

#[derive(Debug, Clone, Default)]
pub struct Block(Vec<Statement>);

#[derive(Debug, Clone)]
pub struct FnParam {
    type_: TypeExpr,
    name: Identifier,
}

#[derive(Debug, Clone)]
pub enum Statement {
    FunctionDeclaration {
        return_type: TypeExpr,
        name: Identifier,
        params: Vec<FnParam>,
        body: Option<Block>,
    },
}
