use rayql::schema::{
    error::ParseError,
    tokenizer::{tokenize, Keyword, Token},
    utils::{get_data_type_with_span, get_model_or_enum_name},
    Argument, Schema,
};

struct TokenConsumer<'a> {
    tokens_iter: std::iter::Peekable<std::slice::Iter<'a, (Token, usize, usize)>>,
}

impl<'a> TokenConsumer<'a> {
    fn new(tokens: &'a [(Token, usize, usize)]) -> Self {
        Self {
            tokens_iter: tokens.iter().peekable(),
        }
    }

    fn next(&mut self) -> Option<(&'a Token, usize, usize)> {
        self.tokens_iter
            .next()
            .map(|(token, line, col)| (token, *line, *col))
    }

    fn peek(&mut self) -> Option<(&'a Token, usize, usize)> {
        self.tokens_iter
            .peek()
            .map(|(token, line, col)| (token, *line, *col))
    }
}

pub fn parse(input: &str) -> Result<Schema, ParseError> {
    let tokens = tokenize(input)?;
    let mut models = Vec::new();
    let mut enums = Vec::new();
    let mut identifiers = std::collections::HashSet::new();
    let mut token_consumer = TokenConsumer::new(&tokens);

    while let Some((token, line_number, column)) = token_consumer.next() {
        match token {
            Token::Keyword(Keyword::Enum) => {
                let enum_name =
                    get_model_or_enum_name(&mut token_consumer.tokens_iter, &mut identifiers)?;
                let enum_declaration = parse_enum(enum_name, &mut token_consumer)?;
                enums.push(enum_declaration);
            }
            Token::Keyword(Keyword::Model) => {
                let model_name =
                    get_model_or_enum_name(&mut token_consumer.tokens_iter, &mut identifiers)?;
                let model_declaration = parse_model(model_name, &mut token_consumer)?;
                models.push(model_declaration);
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number,
                    column,
                })
            }
        }
    }

    Ok(Schema::new(enums, models))
}

fn parse_enum(
    enum_name: String,
    token_consumer: &mut TokenConsumer,
) -> Result<rayql::schema::Enum, ParseError> {
    let mut variants = vec![];
    let mut existing_variants = std::collections::HashSet::new();

    while let Some((token, line_number, column)) = token_consumer.next() {
        match token {
            Token::BraceClose => {
                return Ok(rayql::schema::Enum::new(
                    enum_name,
                    variants,
                    line_number,
                    column,
                ))
            }
            Token::Identifier(variant) => {
                if !existing_variants.insert(variant) {
                    return Err(ParseError::EnumVariantAlreadyExists {
                        variant: variant.clone(),
                        r#enum: enum_name,
                        line_number,
                        column,
                    });
                }

                variants.push(rayql::schema::EnumVariant::new(
                    variant.clone(),
                    line_number,
                    column,
                ))
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number,
                    column,
                })
            }
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

fn parse_model(
    model_name: String,
    token_consumer: &mut TokenConsumer,
) -> Result<rayql::schema::Model, ParseError> {
    let mut fields = vec![];
    let mut field_names = std::collections::HashSet::new();

    while let Some((token, line_number, column)) = token_consumer.next() {
        match token {
            Token::BraceClose => {
                return Ok(rayql::schema::Model::new(
                    model_name,
                    fields,
                    line_number,
                    column,
                ))
            }
            Token::Identifier(identifier) => match token_consumer.next() {
                Some((Token::Colon, _, _)) => {
                    if !field_names.insert(identifier) {
                        return Err(ParseError::FieldAlreadyExistsOnModel {
                            field: identifier.clone(),
                            model: model_name,
                            line_number,
                            column,
                        });
                    }

                    let field = parse_field(identifier.clone(), token_consumer)?;
                    fields.push(field);
                }
                Some((token, line_number, column)) => {
                    return Err(ParseError::UnexpectedToken {
                        token: token.clone(),
                        line_number,
                        column,
                    });
                }
                None => return Err(ParseError::UnexpectedEndOfTokens),
            },
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number,
                    column,
                })
            }
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

fn parse_field(
    name: String,
    token_consumer: &mut TokenConsumer,
) -> Result<rayql::schema::Field, ParseError> {
    let data_type = get_data_type_with_span(token_consumer.next())?;

    let mut properties = vec![];

    while let Some((token, line_number, column)) = token_consumer.peek() {
        match token {
            Token::Comma => {
                token_consumer.next();
                return Ok(rayql::schema::Field::new(
                    name,
                    data_type,
                    properties,
                    line_number,
                    column,
                ));
            }
            Token::Identifier(identifier) => {
                token_consumer.next();
                if let Some((Token::ParenOpen, _, _)) = token_consumer.peek() {
                    token_consumer.next();
                    properties.push(rayql::schema::Property::FunctionCall(parse_function_call(
                        identifier.clone(),
                        rayql::schema::FunctionCallContext::new(name.clone(), data_type.clone()),
                        token_consumer,
                        line_number,
                        column,
                    )?));
                    continue;
                }

                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number,
                    column,
                });
            }
            Token::Keyword(keyword) => {
                token_consumer.next();
                properties.push(rayql::schema::utils::keyword_to_property_value(
                    keyword.clone(),
                    &line_number,
                    &column,
                )?);
            }
            Token::BraceClose => {
                return Ok(rayql::schema::Field::new(
                    name,
                    data_type,
                    properties,
                    line_number,
                    column,
                ))
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number,
                    column,
                })
            }
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

fn parse_function_call(
    name: String,
    context: rayql::schema::FunctionCallContext,
    token_consumer: &mut TokenConsumer,
    fn_call_line: usize,
    fn_call_column: usize,
) -> Result<rayql::schema::FunctionCall, ParseError> {
    let mut arguments: Vec<rayql::schema::Argument> = vec![];

    while let Some((token, line_number, column)) = token_consumer.next() {
        let argument = match token {
            Token::ParenClose => {
                return Ok(rayql::schema::FunctionCall::new(
                    name,
                    arguments,
                    context,
                    fn_call_line,
                    fn_call_column,
                ));
            }
            Token::Identifier(identifier) => {
                if let Some((Token::ParenOpen, _, _)) = token_consumer.peek() {
                    token_consumer.next();
                    Argument::new(
                        rayql::schema::ArgumentValue::FunctionCall(parse_function_call(
                            identifier.clone(),
                            rayql::schema::FunctionCallContext::new(
                                name.clone(),
                                context.property_data_type.clone(),
                            ),
                            token_consumer,
                            line_number,
                            column - identifier.len(),
                        )?),
                        line_number,
                        column,
                    )
                } else {
                    Argument::new(
                        rayql::schema::ArgumentValue::Identifier(identifier.clone()),
                        line_number,
                        column,
                    )
                }
            }
            Token::Reference(entity, property) => {
                if property.contains('.') {
                    return Err(ParseError::InvalidReference {
                        entity: entity.to_string(),
                        property: property.to_string(),
                        line_number,
                        column,
                    });
                }

                Argument::new(
                    rayql::schema::ArgumentValue::Reference(rayql::schema::Reference::new(
                        entity.clone(),
                        property.clone(),
                        line_number,
                        column,
                    )),
                    line_number,
                    column,
                )
            }
            Token::StringLiteral(s) => Argument::new(
                rayql::schema::ArgumentValue::Value(rayql::value::Value::StringLiteral(
                    s.to_string(),
                )),
                line_number,
                column,
            ),
            Token::Integer(i) => Argument::new(
                rayql::schema::ArgumentValue::Value(rayql::value::Value::Integer(i.to_owned())),
                line_number,
                column,
            ),
            Token::Real(r) => Argument::new(
                rayql::schema::ArgumentValue::Value(rayql::value::Value::Real(r.to_owned())),
                line_number,
                column,
            ),
            Token::Boolean(b) => Argument::new(
                rayql::schema::ArgumentValue::Value(rayql::value::Value::Boolean(b.to_owned())),
                line_number,
                column,
            ),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    token: token.clone(),
                    line_number,
                    column,
                })
            }
        };

        if let Some((token, line_number, column)) = token_consumer.peek() {
            match token {
                Token::Comma => {
                    token_consumer.next();
                    arguments.push(argument)
                }
                Token::ParenClose => {
                    token_consumer.next();
                    arguments.push(argument);
                    return Ok(rayql::schema::FunctionCall::new(
                        name,
                        arguments,
                        context,
                        fn_call_line,
                        fn_call_column,
                    ));
                }
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        token: token.clone(),
                        line_number,
                        column,
                    });
                }
            }
        }
    }

    Err(ParseError::UnexpectedEndOfTokens)
}
