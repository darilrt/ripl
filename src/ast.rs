#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    String(String),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}
