#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    String,
    Integer,
    Real,
    Blob,
    Boolean,
    Timestamp,
    Optional(Box<DataType>),
    Enum(String),
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::String => write!(f, "String"),
            DataType::Integer => write!(f, "Integer"),
            DataType::Real => write!(f, "Real"),
            DataType::Blob => write!(f, "Blob"),
            DataType::Boolean => write!(f, "Boolean"),
            DataType::Timestamp => write!(f, "Timestamp"),
            DataType::Optional(inner) => write!(f, "Optional<{}>", inner),
            DataType::Enum(name) => write!(f, "Enum({})", name),
        }
    }
}
