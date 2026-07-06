pub mod primitives;
pub mod statement;

use crate::ast::statement::Statement;

pub trait Generate {
    fn generate(&self, cg: &mut CodeGen);
}

pub struct CodeGen {
    pub output: String,
    pub indent: usize,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
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

    pub fn addln(&mut self, s: &str) {
        self.add(s);
        self.add("\n");
    }

    pub fn add_indented(&mut self, s: &str) {
        let line = format!("{}{}", self.indent_str(), s);
        self.add(&line);
    }

    pub fn add_indentedln(&mut self, s: &str) {
        let line = format!("{}{}\n", self.indent_str(), s);
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

    ($cg:expr, /n $($rest:tt)*) => {{
        $cg.addln("");
        $crate::generate!($cg, $($rest)*);
    }};

    ($cg:expr, /s $($rest:tt)*) => {{
        $cg.add(" ");
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
