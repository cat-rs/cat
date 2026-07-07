use crate::Rule;

#[derive(Debug)]
pub enum ParseError {
    Pest(pest::error::Error<Rule>),
    String(String),
}

#[macro_export]
macro_rules! emit {
    (impl $e:literal, $v:expr) => {
        Err(crate::error::ParseError::String(
            format!("BUG: Unimplemented rule for {}: {:?}", $e, $v),
        ))
    };
}
