use crate::{
    ast::expression::{Expression, Primary},
    code::Generate,
    generate,
};

impl Generate for Expression {
    fn generate(&self, cg: &mut super::CodeGen) {
        match self {
            Expression::Primary(primary) => primary.generate(cg),
            Expression::Ref(expr) => generate!(cg, "&" #expr),
            Expression::BinaryOP { lhs, rhs } => generate!(cg, #lhs " " #rhs),
            Expression::Call { target, args } => {
                generate!(cg, #target "(" for (i, arg) in (args.iter().enumerate()) {
                    if (i > 0) { ", " }
                    #arg
                } ")")
            }
        }
    }
}

impl Generate for Primary {
    fn generate(&self, cg: &mut super::CodeGen) {
        match self {
            Primary::Bool(v) => cg.add(&format!("{v}")),
            Primary::String(v) => cg.add(&format!("{v}")),
            Primary::Float(v) => cg.add(&format!("{v}")),
            Primary::Int(v) => cg.add(&format!("{v}")),
            Primary::Path(path) => path.generate(cg),
        }
    }
}
