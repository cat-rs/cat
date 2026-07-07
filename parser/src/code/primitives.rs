use crate::{
    ast::primitives::{Identifier, Path, TypeExpr},
    code::{Generate, PreGen},
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
            TypeExpr::Path(p) => p.generate(cg),
            TypeExpr::Ptr(t) => generate!(cg, #t "*"),
            TypeExpr::Ref(t) => generate!(cg, #t "*"),

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

impl Generate for i32 {
    fn generate(&self, cg: &mut super::CodeGen) {
        cg.add(&self.to_string());
    }
}

impl PreGen {
    pub fn generate_types(&self) -> String {
        let mut _cg = super::CodeGen::new();

        _cg.pre.shift += self.shift + self.type_defs.len();

        let cg = &mut _cg;

        for (ty, i) in &self.type_defs {
            match ty {
                TypeExpr::Array(ty, len) => {
                    generate!(cg, "typedef struct { \n"
                        /i+
                        if (len.is_none()) {
                            /i "size_t len;\n"
                        }
                        /i #ty " arr" "[" #len "];\n"
                        /i-
                        "} _cat_ty_" #i ";\n")
                }
                TypeExpr::Fn(ty, args) => {
                    generate!(cg, "typedef " #ty " (*_cat_ty_" #i ")" "("
                        for (i, arg) in (args.iter().enumerate()) {
                            if (i > 0) { ", " }
                            #arg
                        }
                    ");\n")
                }
                _ => generate!(cg, "typedef " #ty " _cat_ty_" #i ";\n"),
            }
        }

        if !cg.pre.type_defs.is_empty() {
            cg.pre.generate_types() + &cg.output
        } else {
            _cg.output
        }
    }
}
