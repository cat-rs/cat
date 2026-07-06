mod ast;
mod parser;

fn main() {
    let statements = parser::parse("int main() {}").unwrap();
    println!("{:?}", statements);
}
