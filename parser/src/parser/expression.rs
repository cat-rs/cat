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

impl_ast! {Expression; pair => inner;
    Rule::expression => {
        let mut expr = Expression::try_from(inner.next().unwrap())?;

        for bin_op in inner {
            let mut bin_op_inner = bin_op.into_inner();

            expr = Expression::BinaryOP {
                lhs: Box::new(expr),
                op: bin_op_inner.next().unwrap().to_string(),
                rhs: Box::new(Expression::try_from(bin_op_inner.next().unwrap())?),
            };
        }

        Ok(expr)
    }

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

    Rule::ref_term => Expression::try_from(pair).map(Box::new).map(Expression::Ref)
}
