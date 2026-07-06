pub struct Identifier(pub String);

pub struct Path(pub Vec<Identifier>);

pub enum TypeExpr {
    Path(Path),
    Ref(Box<TypeExpr>),
    Ptr(Box<TypeExpr>),
    Fn(Box<TypeExpr>, Vec<TypeExpr>),
}
