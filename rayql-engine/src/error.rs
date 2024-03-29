use rayql::{
    schema::error::{ParseError, TokenizationError},
    sql::error::{FunctionError, ToSQLError},
};

// TODO: Move these to specific crates

pub fn generate_error_message(error: &ParseError, code: &str) -> String {
    match error {
        ParseError::TokenizationError(tokenization_error) => {
            generate_tokenization_error_message(tokenization_error, code)
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
            "Field '{field}' already exists on model '{model}' at line {}, column {}. Cannot redeclare.",
            line_number, column,
        ),
        ParseError::EnumVariantAlreadyExists { r#enum, variant, line_number, column } => format!(
            "Enum variant '{variant}' already exists on enum '{enum}' at line {}, column {}. Cannot redeclare.",
            line_number, column,
        ),
    }
}

fn generate_tokenization_error_message(
    tokenization_error: &TokenizationError,
    code: &str,
) -> String {
    match tokenization_error {
        TokenizationError::UnexpectedCharacter { char, line, column }
        | TokenizationError::UnknownEscapeSequence { char, line, column } => {
            generate_character_error_message(*char, *line, *column, code)
        }
        TokenizationError::StringLiteralOpened { line, column } => {
            format!("String literal opened at line {}, column {}", line, column)
        }
        TokenizationError::UnexpectedEndOfInput => "Unexpected end of input".to_string(),
    }
}

fn generate_character_error_message(char: char, line: usize, column: usize, code: &str) -> String {
    let mut formatted_code = String::new();
    for (line_number, line_content) in code.lines().enumerate() {
        if line_number + 1 == line {
            formatted_code.push_str(&format!(
                "Error: Unexpected character '{}' at line {}, column {}\n",
                char, line, column
            ));
            formatted_code.push_str(&format!("{}\n", line_content));
            let caret_spacing = " ".repeat(column - 1);
            formatted_code.push_str(&format!("{}^", caret_spacing));
        }
    }
    formatted_code
}

pub fn pretty_to_sql_error_message(error: ToSQLError, code: &str) -> String {
    match error {
        ToSQLError::UnknownReference {
            entity_name,
            line_number,
            column,
        } => {
            format!(
                "Unknown reference: {} at line {}, column {}",
                entity_name, line_number, column
            )
        }
        ToSQLError::EnumNotFound {
            enum_name,
            line_number,
            column,
        } => {
            format!(
                "Enum not found: {} at line {}, column {}",
                enum_name, line_number, column
            )
        }
        ToSQLError::ModelNotFound {
            model_name,
            line_number,
            column,
        } => {
            format!(
                "Model not found: {} at line {}, column {}",
                model_name, line_number, column
            )
        }
        ToSQLError::FieldNotFound {
            model_name,
            field_name,
            line_number,
            column,
        } => {
            format!(
                "Field '{}' does not exists on model '{}': at line {}, column {}",
                field_name, model_name, line_number, column
            )
        }
        ToSQLError::VariantNotFound {
            enum_name,
            variant,
            line_number,
            column,
        } => {
            format!(
                "Variant '{}' does not exists on enum '{}': at line {}, column {}",
                variant, enum_name, line_number, column
            )
        }
        ToSQLError::ConversionError {
            reason,
            line_number,
            column,
        } => {
            format!(
                "Conversion error: {} at line {}, column {}",
                reason, line_number, column
            )
        }
        ToSQLError::FunctionError {
            source,
            line_number,
            column,
        } => pretty_function_error_message(source, code, line_number, column),
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
            "Missing argument at line {}, column {}",
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
