use rayql::schema::{
    error::ParseError,
    tokenizer::{Keyword, Token},
    DataTypeWithSpan,
};

pub(crate) fn get_data_type_with_span(
    input: Option<&(Token, usize, usize)>,
) -> Result<DataTypeWithSpan, ParseError> {
    if let Some((token, line_number, column)) = input {
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
            Token::Optional(token) => {
                let inner_data_type =
                    get_data_type_with_span(Some(&(*token.clone(), *line_number, *column)))?;
                rayql::types::DataType::Optional(Box::new(inner_data_type.data_type))
            }
            Token::Identifier(identifier) => rayql::types::DataType::Enum(identifier.clone()),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number: *line_number,
                    column: *column,
                })
            }
        };

        let data_type_with_span = DataTypeWithSpan::new(data_type, *line_number, *column);

        return Ok(data_type_with_span);
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

pub(crate) fn get_model_or_enum_name(
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<(Token, usize, usize)>>,
    identifiers: &mut std::collections::HashSet<String>,
) -> Result<String, ParseError> {
    let name = match tokens_iter.next() {
        Some((Token::Identifier(name), line_number, column)) => {
            if identifiers.contains(name) {
                return Err(ParseError::IdentifierAlreadyInUse {
                    identifier: name.clone(),
                    line_number: *line_number,
                    column: *column,
                });
            }

            name.clone()
        }
        Some((token, line_number, column)) => {
            return Err(ParseError::UnexpectedToken {
                token: token.clone(),
                line_number: *line_number,
                column: *column,
            })
        }
        None => return Err(ParseError::UnexpectedEndOfTokens),
    };

    identifiers.insert(name.clone());

    match tokens_iter.next() {
        Some((Token::BraceOpen, _, _)) => Ok(name),
        Some((token, line_number, column)) => Err(ParseError::UnexpectedToken {
            token: token.clone(),
            line_number: *line_number,
            column: *column,
        }),
        None => Err(ParseError::UnexpectedEndOfTokens),
    }
}

pub(crate) fn keyword_to_property_value(
    keyword: Keyword,
    line_number: &usize,
    column: &usize,
) -> Result<rayql::schema::Property, ParseError> {
    match keyword {
        Keyword::PrimaryKey => Ok(rayql::schema::Property::PrimaryKey),
        Keyword::AutoIncrement => Ok(rayql::schema::Property::AutoIncrement),
        Keyword::Unique => Ok(rayql::schema::Property::Unique),
        _ => Err(ParseError::UnexpectedToken {
            token: Token::Keyword(keyword),
            line_number: *line_number,
            column: *column,
        }),
    }
}
