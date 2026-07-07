use cat_parser::code::CodeGen;

fn main() {
    let source = std::fs::read_to_string("cat/main.cat").expect("Failed to read src file");

    let statements = cat_parser::parse(&source).unwrap();
    let mut cg = CodeGen::new();

    cg.generate(statements);

    println!("{}", cg.output)
}
