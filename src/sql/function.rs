use rayql::{
    schema::{ArgumentValue, Arguments, Schema},
    sql::error::{FunctionError, ToSQLError},
};

pub fn min(
    schema: &Schema,
    property_name: impl Into<String>,
    arguments: &Arguments,
) -> Result<String, ToSQLError> {
    let argument = get_single_argument("min", arguments)?;

    let min_value = match argument.value {
        ArgumentValue::Value(value) => Ok(value.to_sql()),
        ArgumentValue::FunctionCall(func) => func.to_sql(schema),
        _ => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::InvalidArgument(format!(
                    "min value must be a value, got {:?}",
                    argument.value
                )),
                line_number: argument.line_number,
                column: argument.column,
            })
        }
    }?;

    Ok(format!("CHECK({} >= {})", property_name.into(), min_value))
}

pub fn max(
    schema: &Schema,
    property_name: impl Into<String>,
    arguments: &Arguments,
) -> Result<String, ToSQLError> {
    let argument = get_single_argument("min", arguments)?;

    let max_value = match argument.value {
        ArgumentValue::Value(value) => Ok(value.to_sql()),
        ArgumentValue::FunctionCall(func) => func.to_sql(schema),
        _ => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::InvalidArgument(format!(
                    "max value must be a value, got {:?}",
                    argument.value
                )),
                line_number: argument.line_number,
                column: argument.column,
            })
        }
    }?;

    Ok(format!("CHECK({} <= {})", property_name.into(), max_value))
}

pub fn foreign_key(schema: &Schema, arguments: &Arguments) -> Result<String, ToSQLError> {
    let argument = get_single_argument("foreign_key", arguments)?;

    let reference = match argument.value {
        ArgumentValue::Reference(reference) => reference.field_reference_to_sql(schema)?,
        _ => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::InvalidArgument(format!(
                    "foreign key value must be a reference, got {:?}",
                    argument.value
                )),
                line_number: argument.line_number,
                column: argument.column,
            })
        }
    };

    Ok(format!("REFERENCES {}", reference))
}

pub fn default(schema: &Schema, arguments: &Arguments) -> Result<String, ToSQLError> {
    let argument = get_single_argument("default", arguments)?;

    let value = match argument.value {
        ArgumentValue::Value(value) => Ok(value.to_sql()),
        ArgumentValue::FunctionCall(func) => func.to_sql(schema),
        ArgumentValue::Reference(reference) => reference.variant_reference_to_sql(schema),
        _ => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::InvalidArgument(format!(
                    "default value must be a value, got {:?}",
                    argument.value,
                )),
                line_number: argument.line_number,
                column: argument.column,
            })
        }
    }?;

    Ok(format!("DEFAULT {}", value))
}

fn get_single_argument(
    func: &str,
    arguments: &Arguments,
) -> Result<rayql::schema::Argument, ToSQLError> {
    match arguments.list.as_slice() {
        [arg] => Ok(arg.clone()),
        _ => Err(ToSQLError::FunctionError {
            source: if arguments.list.is_empty() {
                FunctionError::MissingArgument
            } else {
                FunctionError::ExpectsExactlyOneArgument(func.to_string())
            },
            line_number: arguments.line_number,
            column: arguments.column,
        }),
    }
}
