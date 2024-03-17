use std::fmt;

#[derive(Debug)]
pub enum ToSQLError {
    UnknownReference {
        entity_name: String,
        line_number: usize,
        column: usize,
    },
    EnumNotFound {
        enum_name: String,
        line_number: usize,
        column: usize,
    },
    ModelNotFound {
        model_name: String,
        line_number: usize,
        column: usize,
    },
    FieldNotFound {
        model_name: String,
        field_name: String,
        line_number: usize,
        column: usize,
    },
    VariantNotFound {
        enum_name: String,
        variant: String,
        line_number: usize,
        column: usize,
    },
    ConversionError {
        reason: String,
        line_number: usize,
        column: usize,
    },
    FunctionError {
        source: rayql::sql::error::FunctionError,
        line_number: usize,
        column: usize,
    },
}

impl fmt::Display for ToSQLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToSQLError::UnknownReference {
                entity_name,
                line_number,
                column,
            } => {
                write!(
                    f,
                    "Unknown reference: {} at line {line_number}, column {column}",
                    entity_name
                )
            }
            ToSQLError::EnumNotFound {
                enum_name,
                line_number,
                column,
            } => {
                write!(
                    f,
                    "Enum not found: {} at line {line_number}, column {column}",
                    enum_name
                )
            }
            ToSQLError::ModelNotFound {
                model_name,
                line_number,
                column,
            } => {
                write!(
                    f,
                    "Enum not found: {} at line {line_number}, column {column}",
                    model_name
                )
            }
            ToSQLError::FieldNotFound {
                model_name,
                field_name,
                line_number,
                column,
            } => {
                write!(
                    f,
                    "Field '{}' does not exists on model '{}': line {line_number}, column {column}",
                    model_name, field_name
                )
            }
            ToSQLError::VariantNotFound {
                enum_name,
                variant,
                line_number,
                column,
            } => {
                write!(
                    f,
                    "Variant '{}' does not exists on enum '{}': line {line_number}, column {column}",
                    enum_name, variant
                )
            }
            ToSQLError::ConversionError {
                reason,
                line_number,
                column,
            } => {
                write!(
                    f,
                    "Conversion error: {} at line {line_number}, column {column}",
                    reason
                )
            }
            ToSQLError::FunctionError {
                source,
                line_number,
                column,
            } => {
                write!(
                    f,
                    "Function error: {} at line {line_number}, column {column}",
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
