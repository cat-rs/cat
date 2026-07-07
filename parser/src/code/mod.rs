pub mod expression;
pub mod primitives;
pub mod statement;

use std::collections::HashMap;

use crate::ast::{primitives::TypeExpr, statement::Statement};

pub trait Generate {
    fn generate(&self, cg: &mut CodeGen);
}

pub struct PreGen {
    pub type_defs: HashMap<TypeExpr, usize>,
}

pub struct CodeGen {
    pub pre: PreGen,
    pub output: String,
    pub indent: usize,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            pre: PreGen {
                type_defs: HashMap::new(),
            },
            output: String::new(),
            indent: 0,
        }
    }

    fn indent_str(&self) -> String {
        "    ".repeat(self.indent)
    }

    pub fn add(&mut self, s: &str) {
        self.output.push_str(s);
    }

    pub fn add_indented(&mut self, s: &str) {
        let line = format!("{}{}", self.indent_str(), s);
        self.add(&line);
    }

    pub fn generate(&mut self, statements: Vec<Statement>) {
        for stmt in statements {
            stmt.generate(self);
        }
    }
}

impl<T: Generate> Generate for Option<T> {
    fn generate(&self, cg: &mut CodeGen) {
        if let Some(v) = self {
            v.generate(cg);
        }
    }
}

impl PreGen {
    pub fn type_def(&mut self, ty: TypeExpr) -> String {
        let len = self.type_defs.len();

        let id = self.type_defs.entry(ty).or_insert(len);

        format!("_cat_ty_{id}")
    }
}

#[macro_export]
macro_rules! generate {
    ($cg:expr, #$($i:ident).+ $($rest:tt)*) => {{
        $($i).*.generate($cg);
        $crate::generate!($cg, $($rest)*);
    }};

    ($cg:expr, gen $b:block $($rest:tt)*) => {{
        $b

        $crate::generate!($cg, $($rest)*);
    }};

    ($cg:expr, if (let $pat:pat = $expr:expr) { $($inner:tt)* } else { $($else:tt)* } $($rest:tt)*) => {{
        if let $pat = $expr {
            generate!($cg, $($inner)*);
        } else {
            generate!($cg, $($else)*);
        }
        generate!($cg, $($rest)*);
    }};

    ($cg:expr, if ($e:expr) { $($inner:tt)* } else { $($else:tt)* }  $($rest:tt)*) => {{
        if $e {
            $crate::generate!($cg, $($inner)*);
        } else {
            generate!($cg, $($else)*);
        }

        $crate::generate!($cg, $($rest)*);
    }};

    ($cg:expr, if (let $pat:pat = $expr:expr) { $($inner:tt)* } $($rest:tt)*) => {{
        if let $pat = $expr {
            generate!($cg, $($inner)*);
        }

        generate!($cg, $($rest)*);
    }};

    ($cg:expr, if ($e:expr) { $($inner:tt)* }  $($rest:tt)*) => {{
        if $e {
            $crate::generate!($cg, $($inner)*);
        }

        $crate::generate!($cg, $($rest)*);
    }};

    ($cg:expr, for $p:pat in ($e:expr) { $($inner:tt)* } $($rest:tt)*) => {{
        for $p in $e {
            $crate::generate!($cg, $($inner)*);
        }

        $crate::generate!($cg, $($rest)*);
    }};

    ($cg:expr, /i+ $($rest:tt)*) => {{
        $cg.indent += 1;
        $crate::generate!($cg, $($rest)*);
    }};

    ($cg:expr, /i- $($rest:tt)*) => {{
        $cg.indent -= 1;
        $crate::generate!($cg, $($rest)*);
    }};

    ($cg:expr, /i $($rest:tt)*) => {{
        $cg.add_indented("");
        $crate::generate!($cg, $($rest)*);
    }};

    ($cg:expr, $s:literal $($rest:tt)*) => {{
        $cg.add($s);
        $crate::generate!($cg, $($rest)*);
    }};

    ($cg:expr,) => {};
}
