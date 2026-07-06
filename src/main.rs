mod ast;
mod code;
mod parser;

fn main() {
    let statements = parser::parse("int main() {}").unwrap();
    println!("{:?}", statements);
}
