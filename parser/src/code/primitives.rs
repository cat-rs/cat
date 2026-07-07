use crate::{
    ast::primitives::{Identifier, Path, TypeExpr}, code::{Generate, PreGen}, generate,
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
            TypeExpr::Path(p) => p.generate(cg),
            TypeExpr::Ptr(t) => generate!(cg, #t "*"),
            TypeExpr::Ref(t) => generate!(cg, #t "&"),

            ty @ (TypeExpr::Array(_, _) | TypeExpr::Fn(_, _)) => {
                let def = cg.pre.type_def(ty.clone());
                cg.add(&def)
            }
        }
    }
}

impl Generate for String {
    fn generate(&self, cg: &mut super::CodeGen) {
        cg.add(self);
    }
}

impl Generate for usize {
    fn generate(&self, cg: &mut super::CodeGen) {
        cg.add(&self.to_string());
    }
}

impl PreGen {
    pub fn generate_types(&self) -> String {
        let mut _cg = super::CodeGen::new();

        let cg = &mut _cg;

        for (ty, i) in &self.type_defs {
            match ty {
                _ => generate!(cg, "typedef " #ty " _cat_ty_" #i ";"),
            }
        }

        _cg.output
    }
}
