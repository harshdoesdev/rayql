use rayql::{
    schema::{ArgumentValue, Arguments, Schema},
    sql::error::{FunctionError, ToSQLError},
};

pub fn check_value(
    schema: &Schema,
    context: &rayql::schema::FunctionCallContext,
    arguments: &Arguments,
    check_type: &str,
    operator: &str,
) -> Result<String, ToSQLError> {
    let argument = get_single_argument(check_type, arguments)?;

    let (value, name) = match argument.value {
        ArgumentValue::Value(value) => {
            let name = match context.property_data_type {
                rayql::types::DataType::String => format!("LENGTH({})", context.property_name),
                rayql::types::DataType::Integer | rayql::types::DataType::Real => {
                    context.property_name.clone()
                }
                _ => {
                    return Err(ToSQLError::FunctionError {
                        source: FunctionError::InvalidArgument(format!(
                            "{} value must be a value, got {:?}",
                            check_type, value
                        )),
                        line_number: argument.line_number,
                        column: argument.column,
                    })
                }
            };

            Ok((name, value.to_sql()))
        }
        ArgumentValue::FunctionCall(func) => {
            Ok((context.property_name.clone(), func.to_sql(schema)?))
        }
        _ => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::InvalidArgument(format!(
                    "{} value must be a value, got {:?}",
                    check_type, argument.value
                )),
                line_number: argument.line_number,
                column: argument.column,
            })
        }
    }?;

    Ok(format!("CHECK({} {} {})", name, operator, value))
}

pub fn min(
    schema: &Schema,
    context: &rayql::schema::FunctionCallContext,
    arguments: &Arguments,
) -> Result<String, ToSQLError> {
    check_value(schema, context, arguments, "min", ">=")
}

pub fn max(
    schema: &Schema,
    context: &rayql::schema::FunctionCallContext,
    arguments: &Arguments,
) -> Result<String, ToSQLError> {
    check_value(schema, context, arguments, "max", "<=")
}

pub fn foreign_key(
    schema: &Schema,
    arguments: &Arguments,
    property_name: String,
) -> Result<String, ToSQLError> {
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

    Ok(format!(
        "    FOREIGN KEY ({}) REFERENCES {}",
        property_name, reference
    ))
}

pub fn references(schema: &Schema, arguments: &Arguments) -> Result<String, ToSQLError> {
    let argument = get_single_argument("references", arguments)?;

    let reference = match argument.value {
        ArgumentValue::Reference(reference) => reference.field_reference_to_sql(schema)?,
        _ => {
            return Err(ToSQLError::FunctionError {
                source: FunctionError::InvalidArgument(format!(
                    "references value must be a reference, got {:?}",
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
