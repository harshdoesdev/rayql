use rayql::{
    schema::{ParseError, TokenizationError},
    sql::{FunctionError, ToSQLError},
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
            "\x1b[31mUnexpected token {:?} at line {}, column {}\x1b[0m",
            token, line_number, column
        ),
        ParseError::UnexpectedEndOfTokens => "\x1b[31mUnexpected end of tokens\x1b[0m".to_string(),
    }
}

fn generate_tokenization_error_message(
    tokenization_error: &TokenizationError,
    code: &str,
) -> String {
    match tokenization_error {
        TokenizationError::UnexpectedCharacter { char, line, col }
        | TokenizationError::UnknownEscapeSequence { char, line, col } => {
            generate_character_error_message(*char, *line, *col, code)
        }
        TokenizationError::StringLiteralOpened { line, col } => {
            format!(
                "\x1b[31mString literal opened at line {}, column {}\x1b[0m",
                line, col
            )
        }
        TokenizationError::UnexpectedEndOfInput => {
            "\x1b[31mUnexpected end of input\x1b[0m".to_string()
        }
    }
}

fn generate_character_error_message(char: char, line: usize, col: usize, code: &str) -> String {
    let mut formatted_code = String::new();
    for (line_number, line_content) in code.lines().enumerate() {
        if line_number + 1 == line {
            formatted_code.push_str(&format!(
                "\x1b[31mError: Unexpected character '{}' at line {}, column {}\x1b[0m\n",
                char, line, col
            ));
            formatted_code.push_str(&format!("{}\n", line_content));
            let caret_spacing = " ".repeat(col - 1);
            formatted_code.push_str(&format!("{}^", caret_spacing));
        }
    }
    formatted_code
}

pub fn pretty_to_sql_error_message(error: &ToSQLError, code: &str) -> String {
    match error {
        ToSQLError::EnumNotFound {
            enum_name,
            line_number,
            column_number,
        } => {
            format!(
                "\x1b[31mEnum not found: {} at line {}, column {}\x1b[0m",
                enum_name, line_number, column_number
            )
        }
        ToSQLError::ConversionError {
            reason,
            line_number,
            column_number,
        } => {
            format!(
                "\x1b[31mConversion error: {} at line {}, column {}\x1b[0m",
                reason, line_number, column_number
            )
        }
        ToSQLError::FunctionError {
            source,
            line_number,
            column_number,
        } => pretty_function_error_message(source, code, *line_number, *column_number),
    }
}

fn pretty_function_error_message(
    error: &FunctionError,
    _code: &str,
    line_number: usize,
    column_number: usize,
) -> String {
    match error {
        FunctionError::InvalidArgument(msg) => {
            format!(
                "\x1b[31mInvalid argument: {} at line {}, column {}\x1b[0m",
                msg,
                0,
                0 // Assuming no specific line or column for now
            )
        }
        FunctionError::MissingArgument => format!(
            "\x1b[31mMissing argument at line {}, column {}\x1b[0m",
            line_number, column_number,
        ),
        FunctionError::ExpectsExactlyOneArgument(func) => {
            format!(
                "\x1b[31m{func} takes exactly one argument, error at line {}, column {}\x1b[0m",
                line_number, column_number
            )
        }
        FunctionError::UndefinedFunction(func) => {
            format!(
                "\x1b[31mUndefined function '{func}' called at line {}, column {}\x1b[0m",
                line_number, column_number
            )
        }
    }
}
