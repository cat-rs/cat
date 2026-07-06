use crate::{
    ast::primitives::{Identifier, Path},
    describe, ensure, impl_ast,
    parser::Rule,
};

pub enum TypeExpr {
    Path(Path),
    Ref(Box<TypeExpr>),
    Ptr(Box<TypeExpr>),
    Fn(Box<TypeExpr>, Vec<TypeExpr>),
    Array(Box<TypeExpr>, Option<i32>),
}

impl_ast! {Identifier => pair {
    ensure!(pair, Rule::identifier);

    Ok(Identifier(pair.as_str().to_string()))
}}

impl_ast! {Path => pair {
    ensure!(pair, Rule::path);

    Ok(Path(describe!(pair.into_inner())))
}}

impl_ast! {TypeExpr; inner;
    Rule::type_expr => {
        let mut ty = TypeExpr::try_from(inner.next().unwrap())?;

        for ref_pair in inner {
            let mut px_inner = ref_pair.clone().into_inner();

            match ref_pair.as_rule() {
                Rule::ref_type => {
                    ty = TypeExpr::Ref(Box::new(ty));
                }

                Rule::ptr_type => {
                    ty = TypeExpr::Ptr(Box::new(ty));

                }

                Rule::fn_type => {
                    ty = TypeExpr::Fn(Box::new(ty), describe!(px_inner.next().unwrap().into_inner()));
                }

                Rule::array_type => {
                    ty = TypeExpr::Array(
                        Box::new(ty),
                        px_inner
                            .next()
                            .map(|v| {
                                v.as_str().parse().map_err(|_| crate::parser::error::ParseError {})
                            })
                            .transpose()?,
                    )
                }

                _ => Err(crate::parser::error::ParseError {})?,
            }
        }

        Ok(ty)
    }

    Rule::path_type => {
        Ok(TypeExpr::Path(inner.next().unwrap().try_into()?))
    }
}
