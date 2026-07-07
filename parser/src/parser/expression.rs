use crate::{
    Rule,
    ast::expression::{Expression, Primary},
    describe, impl_ast,
};

impl_ast! {Primary; => pair;
    Rule::path => TryFrom::try_from(pair).map(Primary::Path)
    Rule::integer => Ok(Primary::Int(pair.as_str().parse::<i64>().unwrap()))
    Rule::float => Ok(Primary::Float(pair.as_str().parse::<f64>().unwrap()))
    Rule::boolean => Ok(Primary::Bool(pair.as_str().parse::<bool>().unwrap()))
    Rule::string_lit => Ok(Primary::String(pair.into_inner().as_str().to_string()))
}

impl_ast! {Expression; inner;
    Rule::primary_term => {
        let mut expr = Expression::Primary(Primary::try_from(inner.next().unwrap())?);

        for suffix in inner {
            let sx_inner = suffix.clone().into_inner();

            match suffix.as_rule() {
                Rule::call_sx => {
                    expr = Expression::Call {
                        target:Box::new(expr),
                        args: describe!(sx_inner)
                    };
                }

                _ => Err(crate::error::ParseError {})?,
            }
        }

        Ok(expr)
    }
}
