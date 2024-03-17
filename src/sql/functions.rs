use rayql::{
    schema::{Arguments, PropertyValue, Schema},
    sql::error::{FunctionError, ToSQLError},
};

pub fn min_function(
    schema: &Schema,
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
            PropertyValue::FunctionCall(func) => func.to_sql(schema),
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
    schema: &Schema,
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
            PropertyValue::FunctionCall(func) => func.to_sql(schema),
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

pub fn foreign_key(schema: &Schema, arguments: &Arguments) -> Result<String, ToSQLError> {
    assert_got_single_arg("foreign_key", arguments)?;

    // Todo: Move this to a separate, re-usable function
    let reference = match arguments.first() {
        Some(rayql::schema::Argument {
            value,
            line_number,
            column,
        }) => match value {
            PropertyValue::Reference(reference) => reference.field_reference_to_sql(schema)?,
            _ => {
                return Err(ToSQLError::FunctionError {
                    source: FunctionError::InvalidArgument(
                        "foreign key value must be a reference".to_string(),
                    ),
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
    };

    Ok(format!("REFERENCES {}", reference))
}

pub fn default_fn(schema: &Schema, arguments: &Arguments) -> Result<String, ToSQLError> {
    assert_got_single_arg("default", arguments)?;

    let value = match arguments.first() {
        Some(rayql::schema::Argument {
            value,
            line_number,
            column,
        }) => match value {
            PropertyValue::Value(value) => Ok(value.to_sql()),
            PropertyValue::FunctionCall(func) => func.to_sql(schema),
            PropertyValue::Reference(reference) => reference.variant_reference_to_sql(schema),
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
