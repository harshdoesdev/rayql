#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    StringLiteral(String),
    Integer(i64),
    Real(f64),
    Boolean(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::StringLiteral(s) => write!(f, "'{}'", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Real(r) => write!(f, "{}", r),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}

impl Value {
    pub fn get_type(&self) -> rayql::types::DataType {
        match self {
            Value::StringLiteral(_) => rayql::types::DataType::String,
            Value::Integer(_) => rayql::types::DataType::Integer,
            Value::Real(_) => rayql::types::DataType::Real,
            Value::Boolean(_) => rayql::types::DataType::Boolean,
        }
    }
}
