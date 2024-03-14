#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    StringLiteral(String),
    Text(String),
    Integer(i64),
    Real(f64),
    Boolean(bool),
}
