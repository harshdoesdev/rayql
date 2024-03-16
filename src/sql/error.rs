use std::fmt;

#[derive(Debug)]
pub enum ToSQLError {
    EnumNotFound {
        enum_name: String,
        line_number: usize,
        column_number: usize,
    },
    ConversionError {
        reason: String,
        line_number: usize,
        column_number: usize,
    },
    FunctionError {
        source: rayql::sql::FunctionError,
        line_number: usize,
        column_number: usize,
    },
}

impl fmt::Display for ToSQLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToSQLError::EnumNotFound {
                enum_name,
                line_number,
                column_number,
            } => {
                write!(
                    f,
                    "Enum not found: {} at line {line_number}, column {column_number}",
                    enum_name
                )
            }
            ToSQLError::ConversionError {
                reason,
                line_number,
                column_number,
            } => {
                write!(
                    f,
                    "Conversion error: {} at line {line_number}, column {column_number}",
                    reason
                )
            }
            ToSQLError::FunctionError {
                source,
                line_number,
                column_number,
            } => {
                write!(
                    f,
                    "Function error: {} at line {line_number}, column {column_number}",
                    source
                )
            }
        }
    }
}

impl std::error::Error for ToSQLError {}
