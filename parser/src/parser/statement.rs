use crate::{
    Rule,
    ast::{
        expression::Expression,
        statement::{Block, Statement, VarDecl},
    },
    describe, ensure, impl_ast,
};

impl_ast! {VarDecl => pair {
    ensure!(pair, Rule::var_decl);

    let mut inner = pair.into_inner();

    Ok(VarDecl {
        type_: inner.next().unwrap().try_into()?,
        name: inner.next().unwrap().try_into()?
    })
}}

impl_ast! {Block => pair {
    ensure!(pair, Rule::block);

    Ok(Block(describe!(pair.into_inner())))
}}

impl_ast! {Statement; pair => inner;
    Rule::statement => {
        inner.next().unwrap().try_into()
    }

    Rule::fn_decl => {
        Ok(Statement::FunctionDeclaration {
            return_type: inner.next().unwrap().try_into()?,
            name: inner.next().unwrap().try_into()?,
            params: describe!(inner.next().unwrap().into_inner()),
            body: describe!(?inner)
        })
    }

    Rule::struct_decl => {
        Ok(Statement::StructDeclaration {
            name: inner.next().unwrap().try_into()?,
            fields: describe!(inner)
        })
    }

    Rule::var_decl_statement => {
        Ok(Statement::VarDecl {
            decl: inner.next().unwrap().try_into()?,
            init: describe!(?inner),
        })
    }

    Rule::expression => {
        Expression::try_from(pair).map(Statement::Expression)
    }
}
