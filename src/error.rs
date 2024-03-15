use rayql::schema::ParseError;
use rayql::schema::TokenizationError;

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
        TokenizationError::UnexpectedCharacter { char, line, col } => {
            generate_character_error_message(*char, *line, *col, code)
        }
        TokenizationError::StringLiteralOpened { line, col } => {
            format!("\x1b[31mString literal opened at line {}, column {}\x1b[0m", line, col)
        }
        TokenizationError::UnexpectedEndOfInput => "\x1b[31mUnexpected end of input\x1b[0m".to_string(),
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
