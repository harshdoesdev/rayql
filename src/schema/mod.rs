mod parser;
mod tokenizer;
mod utils;

pub use parser::{parse, ParseError};
pub use tokenizer::TokenizationError;

#[derive(Debug, PartialEq, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyValue {
    Identifier(String),
    FunctionCall(String, Vec<PropertyValue>),
    Value(rayql::value::Value),
    PrimaryKey,
    AutoIncrement,
    Unique,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall(pub String, pub Vec<PropertyValue>);

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub name: String,
    pub data_type: rayql::types::DataType,
    pub properties: Vec<PropertyValue>,
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

    pub fn parse(input: &str) -> Result<rayql::Schema, rayql::schema::ParseError> {
        rayql::schema::parse(input)
    }
}
