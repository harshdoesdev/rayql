#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    String,
    Integer,
    Real,
    Blob,
    Boolean,
    Timestamp,
    Optional(Box<DataType>),
    UserDefined(String),
}
