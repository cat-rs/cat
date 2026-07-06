pub mod error;

pub type Result<T> = std::result::Result<T, error::ParseError>;

use pest::Parser;
use pest_derive::Parser;

use crate::ast::statement::Statement;

#[derive(Parser)]
#[grammar = "./src/parser/grammar.pest"]
struct CatParser;

pub fn parse(source: &str) -> Result<Vec<Statement>> {
    let mut pairs = CatParser::parse(Rule::program, source).map_err(|_| error::ParseError {})?;

    let mut statements = vec![];

    for pair in pairs.next().unwrap().into_inner() {
        if pair.as_rule() != Rule::EOI {
            // statements.push();
        }
    }

    Ok(statements)
}
