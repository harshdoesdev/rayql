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
            "Unexpected token {:?} at line {}, column {}",
            token, line_number, column
        ),
        ParseError::UnexpectedEndOfTokens => "Unexpected end of tokens".to_string(),
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
            format!("String literal opened at line {}, column {}", line, col)
        }
        TokenizationError::UnexpectedEndOfInput => "Unexpected end of input".to_string(),
    }
}

fn generate_character_error_message(char: char, line: usize, col: usize, code: &str) -> String {
    let mut formatted_code = String::new();
    for (line_number, line_content) in code.lines().enumerate() {
        if line_number + 1 == line {
            formatted_code.push_str(&format!(
                "Error: Unexpected character '{}' at line {}, column {}\n",
                char, line, col
            ));
            formatted_code.push_str(&format!("{}\n", line_content));
        }
    }
    formatted_code
}
