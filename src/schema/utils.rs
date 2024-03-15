use rayql::schema::{
    tokenizer::{Keyword, Token},
    ParseError,
};

pub(crate) fn get_data_type(
    input: Option<&(Token, usize, usize)>,
) -> Result<rayql::types::DataType, ParseError> {
    if let Some((token, line_number, col)) = input {
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
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number: line_number.clone(),
                    column: col.clone(),
                })
            }
        };

        return Ok(data_type);
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

pub(crate) fn get_model_or_enum_name(
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<(Token, usize, usize)>>,
) -> Result<String, ParseError> {
    let name = match tokens_iter.next() {
        Some((Token::Identifier(name), _, _)) => name.clone(),
        Some((token, line_number, col)) => {
            return Err(ParseError::UnexpectedToken {
                token: token.clone(),
                line_number: line_number.clone(),
                column: col.clone(),
            })
        }
        None => return Err(ParseError::UnexpectedEndOfTokens),
    };

    match tokens_iter.next() {
        Some((Token::BraceOpen, _, _)) => Ok(name),
        Some((token, line_number, col)) => Err(ParseError::UnexpectedToken {
            token: token.clone(),
            line_number: line_number.clone(),
            column: col.clone(),
        }),
        None => Err(ParseError::UnexpectedEndOfTokens),
    }
}