use rayql::schema::{
    tokenizer::{Keyword, Token},
    ParseError,
};

pub(crate) fn get_data_type(token: Option<&Token>) -> Result<rayql::types::DataType, ParseError> {
    if let Some(token) = token {
        let data_type = match token {
            Token::Keyword(keyword) => match keyword {
                Keyword::String => rayql::types::DataType::String,
                Keyword::Integer => rayql::types::DataType::Integer,
                Keyword::Real => rayql::types::DataType::Real,
                Keyword::Boolean => rayql::types::DataType::Boolean,
                Keyword::Blob => rayql::types::DataType::Blob,
                Keyword::Timestamp => rayql::types::DataType::Timestamp,
                _ => unimplemented!("Unexpected data type"),
            },
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        };

        return Ok(data_type);
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

pub(crate) fn get_model_or_enum_name(
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<String, ParseError> {
    let name = match tokens_iter.next() {
        Some(Token::Identifier(name)) => name.clone(),
        Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
        None => return Err(ParseError::UnexpectedEndOfTokens),
    };

    match tokens_iter.next() {
        Some(Token::BraceOpen) => Ok(name),
        Some(token) => Err(ParseError::UnexpectedToken(token.clone())),
        None => Err(ParseError::UnexpectedEndOfTokens),
    }
}
