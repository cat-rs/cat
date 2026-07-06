use crate::code::CodeGen;

mod ast;
mod code;
mod parser;

fn main() {
    let statements = parser::parse("int main();").unwrap();
    let mut cg = CodeGen::new();

    cg.generate(statements);

    println!("{}", cg.output)
}
