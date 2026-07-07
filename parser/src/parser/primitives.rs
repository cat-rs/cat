use crate::{
    Rule, ast::primitives::{Identifier, Path, TypeExpr}, describe, emit, ensure, impl_ast,
};

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
                                v.as_str().parse().map_err(|_| crate::error::ParseError::String(format!("Failed to parse int")))
                            })
                            .transpose()?,
                    )
                }

                _ => emit!(impl "TypeExpr (ref_pair)", ref_pair.as_rule())?,
            }
        }

        Ok(ty)
    }

    Rule::path_type => {
        Ok(TypeExpr::Path(inner.next().unwrap().try_into()?))
    }
}
