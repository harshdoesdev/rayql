use rayql::{
    schema::{Arguments, PropertyValue},
    sql::{FunctionError, ToSQLError},
};

pub fn min_function(
    property_name: impl Into<String>,
    arguments: &Arguments,
) -> Result<String, ToSQLError> {
    assert_got_single_arg("min", arguments)?;

    let min_value = match arguments.first() {
        Some(rayql::schema::Argument {
            value,
            line_number,
            column,
        }) => match value {
            PropertyValue::Value(value) => Ok(value.to_sql()),
            PropertyValue::FunctionCall(func) => func.to_sql(),
            _ => {
                return Err(ToSQLError::FunctionError {
                    source: FunctionError::InvalidArgument(format!(
                        "min value must be a value, got {:?}",
                        value
                    )),
                    line_number: *line_number,
                    column: *column,
                })
            }
        },
        None => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::MissingArgument,
                line_number: arguments.line_number,
                column: arguments.column,
            })
        }
    }?;

    Ok(format!("CHECK({} >= {})", property_name.into(), min_value))
}

pub fn max_function(
    property_name: impl Into<String>,
    arguments: &Arguments,
) -> Result<String, ToSQLError> {
    assert_got_single_arg("min", arguments)?;

    let max_value = match arguments.first() {
        Some(rayql::schema::Argument {
            value,
            line_number,
            column,
        }) => match value {
            PropertyValue::Value(value) => Ok(value.to_sql()),
            PropertyValue::FunctionCall(func) => func.to_sql(),
            _ => {
                return Err(ToSQLError::FunctionError {
                    source: FunctionError::InvalidArgument(format!(
                        "max value must be a value, got {:?}",
                        value
                    )),
                    line_number: *line_number,
                    column: *column,
                })
            }
        },
        None => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::MissingArgument,
                line_number: arguments.line_number,
                column: arguments.column,
            })
        }
    }?;

    Ok(format!("CHECK({} <= {})", property_name.into(), max_value))
}

pub fn foreign_key(arguments: &Arguments) -> Result<String, ToSQLError> {
    assert_got_single_arg("foreign_key", arguments)?;

    let (reference_table, reference_key) = match arguments.first() {
        Some(rayql::schema::Argument {
            value,
            line_number,
            column,
        }) => match value {
            PropertyValue::Identifier(identifier) => match identifier.split_once('.') {
                Some(v) => v,
                None => {
                    return Err(ToSQLError::FunctionError {
                        source: FunctionError::InvalidArgument(
                            "Reference key not found.".to_string(),
                        ),
                        line_number: *line_number,
                        column: *column,
                    })
                }
            },
            _ => {
                return Err(ToSQLError::FunctionError {
                    source: FunctionError::InvalidArgument(
                        "foreign key value must be an identifer".to_string(),
                    ),
                    line_number: arguments.line_number,
                    column: arguments.column,
                })
            }
        },
        None => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::MissingArgument,
                line_number: arguments.line_number,
                column: arguments.column,
            })
        }
    };

    Ok(format!("REFERENCES {}({})", reference_table, reference_key))
}

pub fn default_fn(arguments: &Arguments) -> Result<String, ToSQLError> {
    assert_got_single_arg("default", arguments)?;

    let value = match arguments.first() {
        Some(rayql::schema::Argument {
            value,
            line_number,
            column,
        }) => match value {
            PropertyValue::Value(value) => Ok(value.to_sql()),
            PropertyValue::FunctionCall(func) => func.to_sql(),
            _ => {
                return Err(ToSQLError::FunctionError {
                    source: FunctionError::InvalidArgument(format!(
                        "default value must be a value, got {:?}",
                        value
                    )),
                    line_number: *line_number,
                    column: *column,
                })
            }
        },
        None => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::MissingArgument,
                line_number: arguments.line_number,
                column: arguments.column,
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
            column: arguments.column,
        }),
    }
}
