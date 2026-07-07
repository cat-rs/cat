use crate::{
    ast::statement::{Block, Statement, VarDecl},
    code::{CodeGen, Generate},
    generate,
};

impl Generate for Block {
    fn generate(&self, cg: &mut CodeGen) {
        generate!(cg,
            "{\n"
                /i+
                for stmt in (&self.0) {
                    /i #stmt "\n"
                }
                /i-
            "}"
        )
    }
}

impl Generate for VarDecl {
    fn generate(&self, cg: &mut CodeGen) {
        self.type_.generate(cg);
        cg.add(" ");
        self.name.generate(cg);
    }
}

impl Generate for Statement {
    fn generate(&self, cg: &mut CodeGen) {
        match self {
            Statement::FunctionDeclaration {
                return_type,
                name,
                params,
                body,
            } => {
                generate!(cg, #return_type " " #name "(" for (i, param) in (params.iter().enumerate()) {
                    if (i > 0) {
                        ", "
                    }

                    #param
                } ")" if (let Some(body) = body) {
                    #body
                } else {
                    ";"
                })
            }

            Statement::StructDeclaration { name, fields } => {
                generate!(cg, "struct " #name for param in (fields) {
                    #param ";\n"
                })
            }

            Statement::Expression(expr) => {
                expr.generate(cg);
                cg.add(";\n");
            }
        }
    }
}
