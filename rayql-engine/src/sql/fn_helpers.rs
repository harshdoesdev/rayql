use rayql::{
    schema::{ArgumentValue, Arguments, FunctionCallContext, Schema},
    sql::error::{FunctionError, ToSQLError},
};

macro_rules! single_arg_fn {
    ($name:ident ($schema:ident, $arg_name:ident, $context:ident) $body:block) => {
        pub fn $name(
            $schema: &Schema,
            arguments: &Arguments,
            $context: &FunctionCallContext,
        ) -> Result<String, ToSQLError> {
            let $arg_name = get_single_argument(stringify!($name), arguments)?;

            $body
        }
    };
}

macro_rules! check_value_fn {
    ($name:ident, $operator:expr) => {
        pub fn $name(
            schema: &Schema,
            context: &rayql::schema::FunctionCallContext,
            arguments: &Arguments,
        ) -> Result<String, ToSQLError> {
            check_value(schema, context, arguments, stringify!($name), $operator)
        }
    };
}

macro_rules! argument_matches {
    ($argument:ident, $( $pattern:pat $( if $guard:expr )? => $result:expr ),+ $(,)?) => {
        match $argument.value {
            $(
                $pattern $( if $guard )? => $result,
            )+
            _ => {
                return Err(ToSQLError::FunctionError {
                    source: FunctionError::InvalidArgument(format!(
                        "Invalid argument: {:?}",
                        $argument.value,
                    )),
                    line_number: $argument.line_number,
                    column: $argument.column,
                });
            }
        }
    };
}

pub(crate) fn get_single_argument(
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

pub(crate) fn check_value(
    schema: &Schema,
    context: &FunctionCallContext,
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
