pub mod expression;
pub mod primitives;
pub mod statement;

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
            return Err(crate::error::ParseError::String(format!(
                "BUG (Ensure): expected ({}), found {:?}",
                stringify!($r),
                $pair.as_rule()
            )))?;
        }
    };
}

/// ```rust
/// impl_ast! {}
/// ```
#[macro_export]
macro_rules! impl_ast {
    ($(<$($g:ty),*>)? $p:ty => $n:ident $e:expr) => {
        impl$(<$($g),*>)? TryFrom<pest::iterators::Pair<'_, crate::Rule>> for $p {
            type Error = crate::error::ParseError;

            fn try_from($n: pest::iterators::Pair<'_, crate::Rule>) -> Result<Self, Self::Error> {
                $e
            }
        }
    };

    ($(<$($g:ty),*>)? $p:ty; $n:ident; $($r:pat => $e:expr)*) => {
        impl$(<$($g),*>)? TryFrom<pest::iterators::Pair<'_, crate::Rule>> for $p {
            type Error = crate::error::ParseError;

            fn try_from(pair: pest::iterators::Pair<'_, crate::Rule>) -> Result<Self, Self::Error> {
                let rule = pair.as_rule();
                let mut $n = pair.into_inner();

                match rule {
                    $( $r => $e, )*

                    _ => Err(crate::error::ParseError::String(
                        format!("BUG: Unimplemented rule for {}: {:?}", stringify!($p), rule),
                    ))?,
                }
            }
        }
    };

    ($(<$($g:ty),*>)? $p:ty; => $n:ident; $($r:pat => $e:expr)*) => {
        impl$(<$($g),*>)? TryFrom<pest::iterators::Pair<'_, crate::Rule>> for $p {
            type Error = crate::error::ParseError;

            fn try_from($n: pest::iterators::Pair<'_, crate::Rule>) -> Result<Self, Self::Error> {
                match $n.as_rule() {
                    $( $r => $e, )*

                    _ => Err(crate::error::ParseError::String(
                        format!("BUG: Unimplemented rule for {}: {:?}", stringify!($p), $n.as_rule()),
                    ))?,
                }
            }
        }
    };

    ($(<$($g:ty),*>)? $p:ty; $n:ident => $n1:ident; $($r:pat => $e:expr)*) => {
        impl$(<$($g),*>)? TryFrom<pest::iterators::Pair<'_, crate::Rule>> for $p {
            type Error = crate::error::ParseError;

            fn try_from($n: pest::iterators::Pair<'_, crate::Rule>) -> Result<Self, Self::Error> {
                let mut $n1 = $n.clone().into_inner();

                match $n.as_rule() {
                    $( $r => $e, )*

                    _ => Err(crate::error::ParseError::String(
                        format!("BUG: Unimplemented rule for {}: {:?}", stringify!($p), $n.as_rule()),
                    ))?,
                }
            }
        }
    };
}
