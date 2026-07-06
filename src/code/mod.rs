pub mod primitives;

use crate::ast::statement::Statement;

pub trait Generate {
    fn generate(&self, cg: &mut CodeGen);
}

pub struct CodeGen {
    output: String,
    indent: usize,
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

#[macro_export]
macro_rules! generate {
    ($cg:expr, #$($i:ident).+ $($rest:tt)*) => {{
        $($i).*.generate($cg);
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

    ($cg:expr, /i $($rest:tt)*) => {{
        $cg.add_indented("");
        $crate::generate!($cg, $($rest)*);
    }};

    ($cg:expr, $t:tt $($rest:tt)*) => {{
        $cg.add(&stringify!($t));
        $crate::generate!($cg, $($rest)*);
    }};

    ($cg:expr,) => {};
}
