use rayql::tokenizer::{tokenize, Keyword, Token, TokenizationError};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Tokenization Error: {0}")]
    TokenizationError(#[from] TokenizationError),
    #[error("Unexpected Token: {0}")]
    UnexpectedToken(Token),
    #[error("Unexpected End of Tokens")]
    UnexpectedEndOfTokens,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    String,
    Integer,
    Real,
    Blob,
    Boolean,
    Timestamp,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub name: String,
    pub data_type: DataType,
    pub properties: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq)]
pub struct Schema {
    pub models: Vec<Model>,
    pub enums: Vec<Enum>,
}

impl Schema {
    pub fn new(models: Vec<Model>, enums: Vec<Enum>) -> Self {
        Schema { models, enums }
    }
}

pub fn parse(input: &str) -> Result<Schema, ParseError> {
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

    Ok(Schema::new(models, enums))
}

fn parse_enum(
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Enum, ParseError> {
    let enum_name = get_name(tokens_iter.next())?;

    assert_definition_begin(tokens_iter.next())?;

    let mut variants = vec![];

    for token in tokens_iter.by_ref() {
        match token {
            Token::BraceClose => break,
            Token::Identifier(variant) => variants.push(variant.clone()),
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        }
    }

    Ok(Enum {
        name: enum_name,
        variants,
    })
}

fn parse_model(
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Model, ParseError> {
    let model_name = get_name(tokens_iter.next())?;

    assert_definition_begin(tokens_iter.next())?;

    let mut fields = vec![];

    while let Some(token) = tokens_iter.next() {
        match token {
            Token::BraceClose => break,
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

    Ok(Model {
        name: model_name,
        fields,
    })
}

fn parse_field(
    name: String,
    tokens_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Field, ParseError> {
    let data_type = get_data_type(tokens_iter.next())?;

    let properties = vec![];

    while let Some(token) = tokens_iter.next() {
        if token.eq(&Token::Comma) {
            break;
        }
    }

    Ok(Field {
        name,
        data_type,
        properties,
    })
}

fn get_data_type(token: Option<&Token>) -> Result<DataType, ParseError> {
    if let Some(token) = token {
        let data_type = match token {
            Token::Keyword(keyword) => match keyword {
                Keyword::String => DataType::String,
                Keyword::Integer => DataType::Integer,
                Keyword::Real => DataType::Real,
                Keyword::Boolean => DataType::Boolean,
                Keyword::Blob => DataType::Blob,
                Keyword::Timestamp => DataType::Timestamp,
                _ => unimplemented!("Unexpected data type"),
            },
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        };

        return Ok(data_type);
    }

    Err(ParseError::UnexpectedEndOfTokens)
}

fn get_name(token: Option<&Token>) -> Result<String, ParseError> {
    match token {
        Some(Token::Identifier(name)) => Ok(name.clone()),
        Some(token) => Err(ParseError::UnexpectedToken(token.clone())),
        None => Err(ParseError::UnexpectedEndOfTokens),
    }
}

fn assert_definition_begin(token: Option<&Token>) -> Result<(), ParseError> {
    match token {
        Some(Token::BraceOpen) => Ok(()),
        Some(token) => Err(ParseError::UnexpectedToken(token.clone())),
        None => Err(ParseError::UnexpectedEndOfTokens),
    }
}
