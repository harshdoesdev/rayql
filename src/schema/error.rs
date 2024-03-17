use rayql::schema::tokenizer::Token;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TokenizationError {
    #[error("Unexpected character '{char}' at line {line}, column {column}")]
    UnexpectedCharacter {
        char: char,
        line: usize,
        column: usize,
    },
    #[error("Unknown Escape Sequence '{char}' at line {line}, column {column}")]
    UnknownEscapeSequence {
        char: char,
        line: usize,
        column: usize,
    },
    #[error("String literal opened at line {line}, column {column}")]
    StringLiteralOpened { line: usize, column: usize },
    #[error("Unexpected End of Input")]
    UnexpectedEndOfInput,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Tokenization Error: {0}")]
    TokenizationError(#[from] TokenizationError),
    #[error("Unexpected Token")]
    UnexpectedToken {
        token: Token,
        line_number: usize,
        column: usize,
    },
    #[error("Identifier is already in use")]
    IdentifierAlreadyInUse {
        identifier: String,
        line_number: usize,
        column: usize,
    },
    #[error("Invalid reference, cannot access '{entity}' of '{property}'")]
    InvalidReference {
        entity: String,
        property: String,
        line_number: usize,
        column: usize,
    },
    #[error("Unexpected End of Tokens")]
    UnexpectedEndOfTokens,
}
