pub mod ast;
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
            statements.push(pair.try_into()?);
        }
    }

    Ok(statements)
}

#[macro_export]
macro_rules! describe {
    (? $e:expr) => {
        $e.next().map(TryInto::try_into).transpose()?
    };

    (? $p:pat, $e:expr) => {
        if matches!($e.peek(), $p) {
            $e.next()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?
                .unwrap()
        } else {
            None
        }
    };

    ($e:expr) => {
        $e.map(TryInto::try_into).collect::<Result<_, _>>()?
    };
}

#[macro_export]
macro_rules! ensure {
    ($pair:expr, $r:pat) => {
        if !matches!($pair.as_rule(), $r) {
            return Err(crate::parser::error::ParseError {});
        }
    };
}

/// ```rust
/// impl_ast! {}
/// ```
#[macro_export]
macro_rules! impl_ast {
    ($(<$($g:ty),*>)? $p:ty => $n:ident $e:expr) => {
        impl$(<$($g),*>)? TryFrom<pest::iterators::Pair<'_, crate::parser::Rule>> for $p {
            type Error = crate::parser::error::ParseError;

            fn try_from($n: pest::iterators::Pair<'_, crate::parser::Rule>) -> Result<Self, Self::Error> {
                $e
            }
        }
    };

    ($(<$($g:ty),*>)? $p:ty; $n:ident; $($r:pat => $e:expr)*) => {
        impl$(<$($g),*>)? TryFrom<pest::iterators::Pair<'_, crate::parser::Rule>> for $p {
            type Error = crate::parser::error::ParseError;

            fn try_from(pair: pest::iterators::Pair<'_, crate::parser::Rule>) -> Result<Self, Self::Error> {
                let rule = pair.as_rule();
                let mut $n = pair.into_inner();

                match rule {
                    $( $r => $e ),*

                    _ => Err(crate::parser::error::ParseError {})
                }
            }
        }
    };
}
