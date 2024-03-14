mod tokenizer;
mod utils;

use tokenizer::{tokenize, Keyword, Token, TokenizationError};
use utils::{get_data_type, get_model_or_enum_name};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Tokenization Error: {0}")]
    TokenizationError(#[from] TokenizationError),
    #[error("Unexpected Token: {0}")]
    UnexpectedToken(Token),
    #[error("Unexpected End of Tokens")]
    UnexpectedEndOfTokens,
}

pub fn parse(input: &str) -> Result<rayql::Schema, ParseError> {
    let tokens = tokenize(input)?;
    let mut models = Vec::new();
    let mut enums = Vec::new();
    let mut tokens_iter = tokens.iter().peekable();

    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Keyword(Keyword::Enum) => {
                let enum_declaration = parse_enum(&mut tokens_iter)?;
                enums.push(enum_declaration);
            }
            Token::Keyword(Keyword::Model) => {
                let model_declaration = parse_model(&mut tokens_iter)?;
                models.push(model_declaration);
            }
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        }
    }

    Ok(rayql::Schema::new(models, enums))
}

fn parse_enum(
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<rayql::Enum, ParseError> {
    let enum_name = get_model_or_enum_name(tokens_iter)?;

    let mut variants = vec![];

    for token in tokens_iter.by_ref() {
        match token {
            Token::BraceClose => {
                return Ok(rayql::Enum {
                    name: enum_name,
                    variants,
                })
            }
            Token::Identifier(variant) => variants.push(variant.clone()),
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

fn parse_model(
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<rayql::Model, ParseError> {
    let model_name = get_model_or_enum_name(tokens_iter)?;

    let mut fields = vec![];

    while let Some(token) = tokens_iter.next() {
        match token {
            Token::BraceClose => {
                return Ok(rayql::Model {
                    name: model_name,
                    fields,
                })
            }
            Token::Identifier(identifier) => match tokens_iter.next() {
                Some(Token::Colon) => {
                    let field = parse_field(identifier.clone(), tokens_iter)?;
                    fields.push(field);
                }
                Some(token) => {
                    return Err(ParseError::UnexpectedToken(token.clone()));
                }
                None => return Err(ParseError::UnexpectedEndOfTokens),
            },
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

fn parse_field(
    name: String,
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<rayql::Field, ParseError> {
    let data_type = get_data_type(tokens_iter.next())?;

    let mut properties = vec![];

    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Comma => {
                return Ok(rayql::Field {
                    name,
                    data_type,
                    properties,
                })
            }
            Token::Identifier(identifier) => {
                if let Some(Token::ParenOpen) = tokens_iter.peek() {
                    tokens_iter.next();
                    properties.push(rayql::schema::PropertyValue::FunctionCall(
                        parse_function_call(identifier.clone(), tokens_iter)?,
                    ));
                    continue;
                }

                properties.push(rayql::schema::PropertyValue::Identifier(identifier.clone()));
            }
            Token::Keyword(Keyword::PrimaryKey) => {
                properties.push(rayql::schema::PropertyValue::PrimaryKey)
            }
            Token::Keyword(Keyword::AutoIncrement) => {
                properties.push(rayql::schema::PropertyValue::AutoIncrement)
            }
            Token::Keyword(Keyword::Unique) => {
                properties.push(rayql::schema::PropertyValue::Unique)
            }
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

fn parse_function_call(
    name: String,
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<rayql::schema::FunctionCall, ParseError> {
    let mut arguments: Vec<rayql::schema::PropertyValue> = vec![];

    while let Some(token) = tokens_iter.next() {
        match token {
            Token::ParenClose => return Ok(rayql::schema::FunctionCall(name, arguments)),
            Token::Identifier(identifier) => {
                if let Some(Token::ParenOpen) = tokens_iter.peek() {
                    tokens_iter.next();
                    arguments.push(rayql::schema::PropertyValue::FunctionCall(
                        parse_function_call(identifier.clone(), tokens_iter)?,
                    ));
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
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}
