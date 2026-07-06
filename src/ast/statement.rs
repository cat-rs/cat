use crate::ast::primitives::{Identifier, TypeExpr};

#[derive(Debug, Clone, Default)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug, Clone)]
pub struct FnParam {
    pub type_: TypeExpr,
    pub name: Identifier,
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
