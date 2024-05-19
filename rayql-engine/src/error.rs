use rayql::{
    schema::error::{ParseError, TokenizationError},
    sql::error::{FunctionError, ToSQLError},
};

pub fn pretty_error_message(error: &ParseError, code: &str) -> String {
    match error {
        ParseError::TokenizationError(tokenization_error) => {
            pretty_tokenization_error_message(tokenization_error, code)
        }
        ParseError::UnexpectedToken {
            token,
            line_number,
            column,
        } => format!(
            "Unexpected token {} at line {}, column {}",
            token, line_number, column
        ),
        ParseError::InvalidReference {
            entity,
            property,
            line_number,
            column,
        } => format!(
            "Invalid Reference: Cannot access '{}' of '{}' at line {}, column {}",
            property, entity, line_number, column
        ),
        ParseError::IdentifierAlreadyInUse {
            identifier,
            line_number,
            column,
        } => format!(
            "Cannot re-define '{}' at line {}, column {}",
            identifier, line_number, column
        ),
        ParseError::UnexpectedEndOfTokens => "Unexpected end of tokens".to_string(),
        ParseError::FieldAlreadyExistsOnModel { field, model, line_number, column } => format!(
            "Field '{field}' already exists on model '{model}', cannot redeclare it at line {}, column {}.",
            line_number, column,
        ),
        ParseError::EnumVariantAlreadyExists { r#enum, variant, line_number, column } => format!(
            "Enum variant '{variant}' already exists on enum '{enum}', cannot redeclare it at line {}, column {}.",
            line_number, column,
        ),
    }
}

fn pretty_tokenization_error_message(
    tokenization_error: &TokenizationError,
    _code: &str,
) -> String {
    match tokenization_error {
        TokenizationError::UnexpectedCharacter { char, line, column }
        | TokenizationError::UnknownEscapeSequence { char, line, column } => {
            format!(
                "Unexpected character '{}' at line {}, column {}",
                char, line, column
            )
        }
        TokenizationError::StringLiteralOpened { line, column } => {
            format!("String literal opened at line {}, column {}", line, column)
        }
        TokenizationError::IdentifierBeginsWithDigit {
            identifier,
            line,
            column,
        } => format!(
            "Identifier '{identifier}' cannot begin with a digit at line {}, column {}",
            line, column
        ),
        TokenizationError::UnexpectedEndOfInput => "Unexpected end of input".to_string(),
    }
}

pub fn pretty_to_sql_error_message(error: ToSQLError, code: &str) -> String {
    match error {
        ToSQLError::FunctionError {
            source,
            line_number,
            column,
        } => pretty_function_error_message(source, code, line_number, column),
        e => e.to_string(),
    }
}

fn pretty_function_error_message(
    error: FunctionError,
    _code: &str,
    line_number: usize,
    column: usize,
) -> String {
    match error {
        FunctionError::InvalidArgument(msg) => {
            format!(
                "Invalid argument: {} at line {}, column {}",
                msg, line_number, column,
            )
        }
        FunctionError::MissingArgument => format!(
            "Missing argument to function at line {}, column {}",
            line_number, column,
        ),
        FunctionError::ExpectsExactlyOneArgument(func) => {
            format!(
                "{func} takes exactly one argument, error at line {}, column {}",
                line_number, column
            )
        }
        FunctionError::UndefinedFunction(func) => {
            format!(
                "Undefined function '{func}' called at line {}, column {}",
                line_number, column
            )
        }
    }
}
