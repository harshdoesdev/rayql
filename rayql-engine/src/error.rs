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
            "\x1b[31mUnexpected token {} at line {}, column {}\x1b[0m",
            token, line_number, column
        ),
        ParseError::InvalidReference {
            entity,
            property,
            line_number,
            column,
        } => format!(
            "\x1b[31mInvalid Reference: Cannot access '{}' of '{}' at line {}, column {}\x1b[0m",
            property, entity, line_number, column
        ),
        ParseError::IdentifierAlreadyInUse {
            identifier,
            line_number,
            column,
        } => format!(
            "\x1b[31mCannot re-define '{}' at line {}, column {}\x1b[0m",
            identifier, line_number, column
        ),
        ParseError::UnexpectedEndOfTokens => "\x1b[31mUnexpected end of tokens\x1b[0m".to_string(),
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
            format!(
                "\x1b[31mString literal opened at line {}, column {}\x1b[0m",
                line, column
            )
        }
        TokenizationError::UnexpectedEndOfInput => {
            "\x1b[31mUnexpected end of input\x1b[0m".to_string()
        }
    }
}

fn generate_character_error_message(char: char, line: usize, column: usize, code: &str) -> String {
    let mut formatted_code = String::new();
    for (line_number, line_content) in code.lines().enumerate() {
        if line_number + 1 == line {
            formatted_code.push_str(&format!(
                "\x1b[31mError: Unexpected character '{}' at line {}, column {}\x1b[0m\n",
                char, line, column
            ));
            formatted_code.push_str(&format!("{}\n", line_content));
            let caret_spacing = " ".repeat(column - 1);
            formatted_code.push_str(&format!("{}^", caret_spacing));
        }
    }
    formatted_code
}

pub fn pretty_to_sql_error_message(error: &ToSQLError, code: &str) -> String {
    match error {
        ToSQLError::UnknownReference {
            entity_name,
            line_number,
            column,
        } => {
            format!(
                "\x1b[31mUnknown reference: {} at line {}, column {}\x1b[0m",
                entity_name, line_number, column
            )
        }
        ToSQLError::EnumNotFound {
            enum_name,
            line_number,
            column,
        } => {
            format!(
                "\x1b[31mEnum not found: {} at line {}, column {}\x1b[0m",
                enum_name, line_number, column
            )
        }
        ToSQLError::ModelNotFound {
            model_name,
            line_number,
            column,
        } => {
            format!(
                "\x1b[31mModel not found: {} at line {}, column {}\x1b[0m",
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
                "\x1b[31mField '{}' does not exists on model '{}': at line {}, column {}\x1b[0m",
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
                "\x1b[31mVariant '{}' does not exists on enum '{}': at line {}, column {}\x1b[0m",
                variant, enum_name, line_number, column
            )
        }
        ToSQLError::ConversionError {
            reason,
            line_number,
            column,
        } => {
            format!(
                "\x1b[31mConversion error: {} at line {}, column {}\x1b[0m",
                reason, line_number, column
            )
        }
        ToSQLError::FunctionError {
            source,
            line_number,
            column,
        } => pretty_function_error_message(source, code, *line_number, *column),
    }
}

fn pretty_function_error_message(
    error: &FunctionError,
    _code: &str,
    line_number: usize,
    column: usize,
) -> String {
    match error {
        FunctionError::InvalidArgument(msg) => {
            format!(
                "\x1b[31mInvalid argument: {} at line {}, column {}\x1b[0m",
                msg, line_number, column,
            )
        }
        FunctionError::MissingArgument => format!(
            "\x1b[31mMissing argument at line {}, column {}\x1b[0m",
            line_number, column,
        ),
        FunctionError::ExpectsExactlyOneArgument(func) => {
            format!(
                "\x1b[31m{func} takes exactly one argument, error at line {}, column {}\x1b[0m",
                line_number, column
            )
        }
        FunctionError::UndefinedFunction(func) => {
            format!(
                "\x1b[31mUndefined function '{func}' called at line {}, column {}\x1b[0m",
                line_number, column
            )
        }
    }
}
