use std::fmt;

use super::function_to_sql::FunctionError;

#[derive(Debug)]
pub enum ToSQLError {
    EnumNotFound(String),
    ConversionError(String),
    FunctionError(FunctionError),
}

impl fmt::Display for ToSQLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToSQLError::EnumNotFound(enum_name) => {
                write!(f, "Enum not found: {}", enum_name)
            }
            ToSQLError::ConversionError(reason) => {
                write!(f, "Conversion error: {}", reason)
            }
            ToSQLError::FunctionError(reason) => {
                write!(f, "Function error: {}", reason)
            }
        }
    }
}

impl std::error::Error for ToSQLError {}
