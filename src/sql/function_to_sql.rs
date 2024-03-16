use rayql::{
    schema::{Arguments, PropertyValue},
    sql::ToSQLError,
};

#[derive(Debug)]
pub enum FunctionError {
    InvalidArgument(String),
    MissingArgument,
    MoreThanOneArgumentPassed,
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
            FunctionError::MoreThanOneArgumentPassed => {
                write!(f, "More than one argument passed")
            }
        }
    }
}

impl std::error::Error for FunctionError {}

pub fn min_function(
    property_name: impl Into<String>,
    arguments: &Arguments,
) -> Result<String, ToSQLError> {
    assert_got_single_arg(arguments)?;

    let min_value = match arguments.first() {
        Some(value) => match value {
            PropertyValue::Value(value) => Ok(value.to_sql()),
            PropertyValue::FunctionCall(func) => func.to_sql(),
            _ => {
                return Err(ToSQLError::FunctionError(FunctionError::InvalidArgument(
                    format!("min value must be a value, got {:?}", value),
                )))
            }
        },
        None => return Err(ToSQLError::FunctionError(FunctionError::MissingArgument)),
    }?;

    Ok(format!("CHECK({} >= {})", property_name.into(), min_value))
}

pub fn foreign_key(arguments: &Arguments) -> Result<String, ToSQLError> {
    assert_got_single_arg(arguments)?;

    let (reference_table, reference_key) = match arguments.first() {
        Some(value) => match value {
            PropertyValue::Identifier(identifier) => match identifier.split_once('.') {
                Some(v) => v,
                None => {
                    return Err(ToSQLError::FunctionError(FunctionError::InvalidArgument(
                        "Reference key not found.".to_string(),
                    )))
                }
            },
            _ => {
                return Err(ToSQLError::FunctionError(FunctionError::InvalidArgument(
                    "foreign key value must be an identifer".to_string(),
                )))
            }
        },
        None => return Err(ToSQLError::FunctionError(FunctionError::MissingArgument)),
    };

    Ok(format!("REFERENCES {}({})", reference_table, reference_key))
}

pub fn default_fn(arguments: &Arguments) -> Result<String, ToSQLError> {
    assert_got_single_arg(arguments)?;

    let value = match arguments.first() {
        Some(value) => match value {
            PropertyValue::Value(value) => Ok(value.to_sql()),
            PropertyValue::FunctionCall(func) => func.to_sql(),
            _ => {
                return Err(ToSQLError::FunctionError(FunctionError::InvalidArgument(
                    format!("default value must be a value, got {:?}", value),
                )))
            }
        },
        None => return Err(ToSQLError::FunctionError(FunctionError::MissingArgument)),
    }?;

    Ok(format!("DEFAULT {}", value))
}

fn assert_got_single_arg(arguments: &Arguments) -> Result<(), ToSQLError> {
    match arguments.0.len() {
        1 => Ok(()),
        _ => Err(ToSQLError::FunctionError(
            FunctionError::MoreThanOneArgumentPassed,
        )),
    }
}
