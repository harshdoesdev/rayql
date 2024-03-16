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
    FunctionCall(FunctionCall),
    Value(rayql::value::Value),
    PrimaryKey,
    AutoIncrement,
    Unique,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Arguments,
    pub property_name: String,
}

impl FunctionCall {
    pub fn new(property_name: String, name: String, arguments: Vec<PropertyValue>) -> Self {
        FunctionCall {
            name,
            arguments: Arguments::from_vec(arguments),
            property_name,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Arguments(pub Vec<PropertyValue>);

impl Arguments {
    pub fn from_vec(arguments: Vec<PropertyValue>) -> Self {
        Arguments(arguments)
    }

    pub fn first(&self) -> Option<&PropertyValue> {
        self.0.first()
    }

    // nth_of_type
}

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
    pub enums: Vec<Enum>,
    pub models: Vec<Model>,
}

impl Schema {
    pub fn new(enums: Vec<Enum>, models: Vec<Model>) -> Self {
        Schema { enums, models }
    }

    pub fn parse(input: &str) -> Result<rayql::Schema, rayql::schema::ParseError> {
        rayql::schema::parse(input)
    }
}
