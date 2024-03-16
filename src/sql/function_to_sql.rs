use rayql::{
    schema::{Arguments, PropertyValue},
    sql::ToSQLError,
};

#[derive(Debug)]
pub enum FunctionError {
    InvalidArgument(String),
    MissingArgument,
    ExpectsExactlyOneArgument(String),
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
        }
    }
}

impl std::error::Error for FunctionError {}

pub fn min_function(
    property_name: impl Into<String>,
    arguments: &Arguments,
) -> Result<String, ToSQLError> {
    assert_got_single_arg("min", arguments)?;

    let min_value = match arguments.first() {
        Some(value) => match value {
            PropertyValue::Value(value) => Ok(value.to_sql()),
            PropertyValue::FunctionCall(func) => func.to_sql(),
            _ => {
                return Err(ToSQLError::FunctionError {
                    source: FunctionError::InvalidArgument(format!(
                        "min value must be a value, got {:?}",
                        value
                    )),
                    line_number: arguments.line_number.clone(),
                    column_number: arguments.column_number.clone(),
                })
            }
        },
        None => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::MissingArgument,
                line_number: arguments.line_number.clone(),
                column_number: arguments.column_number.clone(),
            })
        }
    }?;

    Ok(format!("CHECK({} >= {})", property_name.into(), min_value))
}

pub fn foreign_key(arguments: &Arguments) -> Result<String, ToSQLError> {
    assert_got_single_arg("foreign_key", arguments)?;

    let (reference_table, reference_key) = match arguments.first() {
        Some(value) => match value {
            PropertyValue::Identifier(identifier) => match identifier.split_once('.') {
                Some(v) => v,
                None => {
                    return Err(ToSQLError::FunctionError {
                        source: FunctionError::InvalidArgument(
                            "Reference key not found.".to_string(),
                        ),
                        line_number: arguments.line_number,
                        column_number: arguments.column_number,
                    })
                }
            },
            _ => {
                return Err(ToSQLError::FunctionError {
                    source: FunctionError::InvalidArgument(
                        "foreign key value must be an identifer".to_string(),
                    ),
                    line_number: arguments.line_number,
                    column_number: arguments.column_number,
                })
            }
        },
        None => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::MissingArgument,
                line_number: arguments.line_number,
                column_number: arguments.column_number,
            })
        }
    };

    Ok(format!("REFERENCES {}({})", reference_table, reference_key))
}

pub fn default_fn(arguments: &Arguments) -> Result<String, ToSQLError> {
    assert_got_single_arg("default", arguments)?;

    let value = match arguments.first() {
        Some(value) => match value {
            PropertyValue::Value(value) => Ok(value.to_sql()),
            PropertyValue::FunctionCall(func) => func.to_sql(),
            _ => {
                return Err(ToSQLError::FunctionError {
                    source: FunctionError::InvalidArgument(format!(
                        "default value must be a value, got {:?}",
                        value
                    )),
                    line_number: arguments.line_number,
                    column_number: arguments.column_number,
                })
            }
        },
        None => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::MissingArgument,
                line_number: arguments.line_number,
                column_number: arguments.column_number,
            })
        }
    }?;

    Ok(format!("DEFAULT {}", value))
}

fn assert_got_single_arg(func: &str, arguments: &Arguments) -> Result<(), ToSQLError> {
    match arguments.list.len() {
        1 => Ok(()),
        _ => Err(ToSQLError::FunctionError {
            source: FunctionError::ExpectsExactlyOneArgument(func.to_string()),
            line_number: arguments.line_number,
            column_number: arguments.column_number,
        }),
    }
}
