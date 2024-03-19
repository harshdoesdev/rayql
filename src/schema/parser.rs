use rayql::schema::{
    error::ParseError,
    tokenizer::{tokenize, Keyword, Token},
    utils::{get_data_type, get_model_or_enum_name},
    Argument, Schema,
};

pub fn parse(input: &str) -> Result<Schema, ParseError> {
    let tokens = tokenize(input)?;
    let mut models = Vec::new();
    let mut enums = Vec::new();
    let mut identifiers = std::collections::HashSet::new();
    let mut tokens_iter = tokens.iter().peekable();

    while let Some((token, line_number, column)) = tokens_iter.next() {
        match token {
            Token::Keyword(Keyword::Enum) => {
                let enum_name = get_model_or_enum_name(&mut tokens_iter, &mut identifiers)?;
                let enum_declaration = parse_enum(enum_name, &mut tokens_iter)?;
                enums.push(enum_declaration);
            }
            Token::Keyword(Keyword::Model) => {
                let model_name = get_model_or_enum_name(&mut tokens_iter, &mut identifiers)?;
                let model_declaration = parse_model(model_name, &mut tokens_iter)?;
                models.push(model_declaration);
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number: *line_number,
                    column: *column,
                })
            }
        }
    }

    Ok(Schema::new(enums, models))
}

fn parse_enum(
    enum_name: String,
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<(Token, usize, usize)>>,
) -> Result<rayql::schema::Enum, ParseError> {
    let mut variants = vec![];

    for (token, line_number, column) in tokens_iter.by_ref() {
        match token {
            Token::BraceClose => {
                return Ok(rayql::schema::Enum::new(
                    enum_name,
                    variants,
                    *line_number,
                    *column,
                ))
            }
            Token::Identifier(variant) => variants.push(rayql::schema::EnumVariant::new(
                variant.clone(),
                *line_number,
                *column,
            )),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number: *line_number,
                    column: *column,
                })
            }
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

fn parse_model(
    model_name: String,
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<(Token, usize, usize)>>,
) -> Result<rayql::schema::Model, ParseError> {
    let mut fields = vec![];

    while let Some((token, line_number, column)) = tokens_iter.next() {
        match token {
            Token::BraceClose => {
                return Ok(rayql::schema::Model::new(
                    model_name,
                    fields,
                    *line_number,
                    *column,
                ))
            }
            Token::Identifier(identifier) => match tokens_iter.next() {
                Some((Token::Colon, _, _)) => {
                    let field = parse_field(identifier.clone(), tokens_iter)?;
                    fields.push(field);
                }
                Some((token, line_number, column)) => {
                    return Err(ParseError::UnexpectedToken {
                        token: token.clone(),
                        line_number: *line_number,
                        column: *column,
                    });
                }
                None => return Err(ParseError::UnexpectedEndOfTokens),
            },
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number: *line_number,
                    column: *column,
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

    while let Some((token, line_number, column)) = tokens_iter.peek() {
        match token {
            Token::Comma => {
                tokens_iter.next();
                return Ok(rayql::schema::Field::new(
                    name,
                    data_type,
                    properties,
                    *line_number,
                    *column,
                ));
            }
            Token::Identifier(identifier) => {
                tokens_iter.next();
                if let Some((Token::ParenOpen, _, _)) = tokens_iter.peek() {
                    tokens_iter.next();
                    properties.push(rayql::schema::Property::FunctionCall(parse_function_call(
                        name.clone(),
                        identifier.clone(),
                        tokens_iter,
                    )?));
                    continue;
                }

                properties.push(rayql::schema::Property::Type(identifier.clone()));
            }
            Token::Keyword(keyword) => {
                tokens_iter.next();
                properties.push(rayql::schema::utils::keyword_to_property_value(
                    keyword.clone(),
                    line_number,
                    column,
                )?);
            }
            Token::BraceClose => {
                return Ok(rayql::schema::Field::new(
                    name,
                    data_type,
                    properties,
                    *line_number,
                    *column,
                ))
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number: *line_number,
                    column: *column,
                })
            }
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

fn parse_function_call(
    property_name: String,
    name: String,
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<(Token, usize, usize)>>,
) -> Result<rayql::schema::FunctionCall, ParseError> {
    let mut arguments: Vec<rayql::schema::Argument> = vec![];

    while let Some((token, line_number, column)) = tokens_iter.next() {
        match token {
            Token::ParenClose => {
                return Ok(rayql::schema::FunctionCall::new(
                    property_name,
                    name,
                    arguments,
                    *line_number,
                    *column,
                ))
            }
            Token::Identifier(identifier) => {
                if let Some((Token::ParenOpen, line_number, column)) = tokens_iter.peek() {
                    tokens_iter.next();
                    arguments.push(Argument::new(
                        rayql::schema::ArgumentValue::FunctionCall(parse_function_call(
                            name.clone(),
                            identifier.clone(),
                            tokens_iter,
                        )?),
                        *line_number,
                        *column,
                    ));
                    continue;
                }

                arguments.push(Argument::new(
                    rayql::schema::ArgumentValue::Identifier(identifier.clone()),
                    *line_number,
                    *column,
                ));
            }
            Token::Reference(entity, property) => {
                if property.contains('.') {
                    return Err(ParseError::InvalidReference {
                        entity: entity.to_string(),
                        property: property.to_string(),
                        line_number: *line_number,
                        column: *column,
                    });
                }

                arguments.push(Argument::new(
                    rayql::schema::ArgumentValue::Reference(rayql::schema::Reference::new(
                        entity.clone(),
                        property.clone(),
                        *line_number,
                        *column,
                    )),
                    *line_number,
                    *column,
                ));
            }
            Token::StringLiteral(s) => arguments.push(Argument::new(
                rayql::schema::ArgumentValue::Value(rayql::value::Value::StringLiteral(
                    s.to_string(),
                )),
                *line_number,
                *column,
            )),
            Token::Integer(i) => arguments.push(Argument::new(
                rayql::schema::ArgumentValue::Value(rayql::value::Value::Integer(i.to_owned())),
                *line_number,
                *column,
            )),
            Token::Real(r) => arguments.push(Argument::new(
                rayql::schema::ArgumentValue::Value(rayql::value::Value::Real(r.to_owned())),
                *line_number,
                *column,
            )),
            Token::Boolean(b) => arguments.push(Argument::new(
                rayql::schema::ArgumentValue::Value(rayql::value::Value::Boolean(b.to_owned())),
                *line_number,
                *column,
            )),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number: *line_number,
                    column: *column,
                })
            }
        }

        if let Some((Token::Comma, _, _)) = tokens_iter.peek() {
            tokens_iter.next();
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}
