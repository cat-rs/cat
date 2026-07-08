#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path(pub Vec<Identifier>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub enum TypeExpr {
    Path(Path),
    Ref(Box<TypeExpr>),
    Ptr(Box<TypeExpr>),
    Fn(Box<TypeExpr>, Vec<TypeExpr>),
    Array(Box<TypeExpr>, Option<i32>),
}
