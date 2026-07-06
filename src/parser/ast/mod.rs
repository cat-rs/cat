pub mod primitives;
pub mod statement;

#[macro_export]
macro_rules! describe {
    (? $e:expr) => {
        $e.next().map(TryInto::try_into).collect::<Result<_, _>>()?
    };

    (? $p:pat, $e:expr) => {
        if matches!($e.peek(), $p) {
            $e.next()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?
                .unwrap()
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
