use rayql::schema::tokenizer::{tokenize, Keyword, Token, TokenizationError};
use rayql::schema::utils::{get_data_type, get_model_or_enum_name};

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
    #[error("Unexpected End of Tokens")]
    UnexpectedEndOfTokens,
}

pub fn parse(input: &str) -> Result<rayql::Schema, ParseError> {
    let tokens = tokenize(input)?;
    let mut models = Vec::new();
    let mut enums = Vec::new();
    let mut tokens_iter = tokens.iter().peekable();

    while let Some((token, line_number, col)) = tokens_iter.next() {
        match token {
            Token::Keyword(Keyword::Enum) => {
                let enum_declaration = parse_enum(&mut tokens_iter)?;
                enums.push(enum_declaration);
            }
            Token::Keyword(Keyword::Model) => {
                let model_declaration = parse_model(&mut tokens_iter)?;
                models.push(model_declaration);
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number: line_number.clone(),
                    column: col.clone(),
                })
            }
        }
    }

    Ok(rayql::Schema::new(enums, models))
}

fn parse_enum(
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<(Token, usize, usize)>>,
) -> Result<rayql::schema::Enum, ParseError> {
    let enum_name = get_model_or_enum_name(tokens_iter)?;

    let mut variants = vec![];

    for (token, line_number, col) in tokens_iter.by_ref() {
        match token {
            Token::BraceClose => {
                return Ok(rayql::schema::Enum {
                    name: enum_name,
                    variants,
                })
            }
            Token::Identifier(variant) => variants.push(variant.clone()),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number: line_number.clone(),
                    column: col.clone(),
                })
            }
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

fn parse_model(
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<(Token, usize, usize)>>,
) -> Result<rayql::schema::Model, ParseError> {
    let model_name = get_model_or_enum_name(tokens_iter)?;

    let mut fields = vec![];

    while let Some((token, line_number, col)) = tokens_iter.next() {
        match token {
            Token::BraceClose => {
                return Ok(rayql::schema::Model {
                    name: model_name,
                    fields,
                })
            }
            Token::Identifier(identifier) => match tokens_iter.next() {
                Some((Token::Colon, _, _)) => {
                    let field = parse_field(identifier.clone(), tokens_iter)?;
                    fields.push(field);
                }
                Some((token, line_number, col)) => {
                    return Err(ParseError::UnexpectedToken {
                        token: token.clone(),
                        line_number: line_number.clone(),
                        column: col.clone(),
                    });
                }
                None => return Err(ParseError::UnexpectedEndOfTokens),
            },
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number: line_number.clone(),
                    column: col.clone(),
                })
            }
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

fn parse_field(
    name: String,
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<(Token, usize, usize)>>,
) -> Result<rayql::schema::Field, ParseError> {
    let data_type = get_data_type(tokens_iter.next())?;

    let mut properties = vec![];

    while let Some((token, line_number, col)) = tokens_iter.next() {
        match token {
            Token::Comma => {
                return Ok(rayql::schema::Field {
                    name,
                    data_type,
                    properties,
                })
            }
            Token::Identifier(identifier) => {
                if let Some((Token::ParenOpen, _, _)) = tokens_iter.peek() {
                    tokens_iter.next();
                    properties.push(parse_function_call(identifier.clone(), tokens_iter)?);
                    continue;
                }

                properties.push(rayql::schema::PropertyValue::Identifier(identifier.clone()));
            }
            Token::Keyword(keyword) => properties.push(
                rayql::schema::utils::keyword_to_property_value(keyword.clone(), line_number, col)?,
            ),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number: line_number.clone(),
                    column: col.clone(),
                })
            }
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

fn parse_function_call(
    name: String,
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<(Token, usize, usize)>>,
) -> Result<rayql::schema::PropertyValue, ParseError> {
    let mut arguments: Vec<rayql::schema::PropertyValue> = vec![];

    while let Some((token, line_number, col)) = tokens_iter.next() {
        match token {
            Token::ParenClose => {
                return Ok(rayql::schema::PropertyValue::FunctionCall(name, arguments))
            }
            Token::Identifier(identifier) => {
                if let Some((Token::ParenOpen, _, _)) = tokens_iter.peek() {
                    tokens_iter.next();
                    arguments.push(parse_function_call(identifier.clone(), tokens_iter)?);
                    continue;
                }

                arguments.push(rayql::schema::PropertyValue::Identifier(identifier.clone()));
            }
            Token::StringLiteral(s) => arguments.push(rayql::schema::PropertyValue::Value(
                rayql::value::Value::StringLiteral(s.to_string()),
            )),
            Token::Integer(i) => arguments.push(rayql::schema::PropertyValue::Value(
                rayql::value::Value::Integer(i.to_owned()),
            )),
            Token::Real(r) => arguments.push(rayql::schema::PropertyValue::Value(
                rayql::value::Value::Real(r.to_owned()),
            )),
            Token::Boolean(b) => arguments.push(rayql::schema::PropertyValue::Value(
                rayql::value::Value::Boolean(b.to_owned()),
            )),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number: line_number.clone(),
                    column: col.clone(),
                })
            }
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}
