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

#[derive(Debug)]
pub enum FunctionError {
    InvalidArgument(String),
    MissingArgument,
    ExpectsExactlyOneArgument(String),
    UndefinedFunction(String),
}

impl std::fmt::Display for FunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionError::InvalidArgument(msg) => {
                write!(f, "Invalid argument: {}", msg)
            }
            FunctionError::MissingArgument => {
                write!(f, "Missing argument")
            }
            FunctionError::ExpectsExactlyOneArgument(func) => {
                write!(f, "{func} exactly one argument")
            }
            FunctionError::UndefinedFunction(func) => {
                write!(f, "Undefined function called '{func}'")
            }
        }
    }
}

impl std::error::Error for FunctionError {}
