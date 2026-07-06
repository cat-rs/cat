use crate::{
    ast::primitives::{Identifier, Path, TypeExpr},
    code::Generate,
    generate,
};

impl Generate for Identifier {
    fn generate(&self, cg: &mut super::CodeGen) {
        cg.add(&self.0);
    }
}

impl Generate for Path {
    fn generate(&self, cg: &mut super::CodeGen) {
        for (i, ident) in self.0.iter().enumerate() {
            if i > 0 {
                cg.add("::");
            }

            ident.generate(cg);
        }
    }
}

impl Generate for TypeExpr {
    fn generate(&self, cg: &mut super::CodeGen) {
        match self {
            TypeExpr::Array(t, _) => t.generate(cg),
            TypeExpr::Fn(t, _) => t.generate(cg),
            TypeExpr::Path(p) => p.generate(cg),
            TypeExpr::Ptr(t) => generate!(cg, #t "*"),
            TypeExpr::Ref(t) => generate!(cg, #t "&"),
        }
    }
}
