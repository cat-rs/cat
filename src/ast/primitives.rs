pub struct Identifier(String);

pub struct Path(Vec<Identifier>);

pub enum TypeExpr {
    Path(Path),
    Ref(Box<TypeExpr>),
    Ptr(Box<TypeExpr>),
    Fn(Box<TypeExpr>, Vec<TypeExpr>),
}
