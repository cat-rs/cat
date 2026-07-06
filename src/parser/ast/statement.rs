use crate::{
    ast::statement::{Block, FnParam, Statement},
    describe, ensure, impl_ast,
    parser::Rule,
};

impl_ast! {FnParam => pair {
    ensure!(pair, Rule::fn_param);

    let mut inner = pair.into_inner();

    Ok(FnParam {
        type_: inner.next().unwrap().try_into()?,
        name: inner.next().unwrap().try_into()?
    })
}}

impl_ast! {Block => pair {
    ensure!(pair, Rule::block);

    Ok(Block(describe!(pair.into_inner())))
}}

impl_ast! {Statement; inner;
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
}
