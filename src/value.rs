#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    StringLiteral(String),
    Integer(i64),
    Real(f64),
    Boolean(bool),
}
