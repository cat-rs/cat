use crate::ast::{
    expression::Expression,
    primitives::{Identifier, TypeExpr},
};

#[derive(Debug, Clone, Default)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub type_: TypeExpr,
    pub name: Identifier,
}

#[derive(Debug, Clone)]
pub enum Statement {
    FunctionDeclaration {
        return_type: TypeExpr,
        name: Identifier,
        params: Vec<VarDecl>,
        body: Option<Block>,
    },

    StructDeclaration {
        name: Identifier,
        fields: Vec<VarDecl>,
    },

    Expression(Expression),
}
