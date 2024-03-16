mod parser;
mod tokenizer;
mod utils;

pub use parser::{parse, ParseError};
pub use tokenizer::TokenizationError;

#[derive(Debug, PartialEq, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub line_number: usize,
    pub column_number: usize,
}

impl Enum {
    pub fn new(
        name: String,
        variants: Vec<EnumVariant>,
        line_number: usize,
        column_number: usize,
    ) -> Self {
        Enum {
            name,
            variants,
            line_number,
            column_number,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub line_number: usize,
    pub column_number: usize,
}

impl EnumVariant {
    pub fn new(name: String, line_number: usize, column_number: usize) -> Self {
        EnumVariant {
            name,
            line_number,
            column_number,
        }
    }
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
    pub line_number: usize,
    pub column_number: usize,
}

impl FunctionCall {
    pub fn new(
        property_name: String,
        name: String,
        arguments: Vec<PropertyValue>,
        line_number: usize,
        column_number: usize,
    ) -> Self {
        FunctionCall {
            name,
            arguments: Arguments::from_vec(arguments, line_number.clone(), column_number.clone()),
            property_name,
            line_number,
            column_number,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Arguments {
    pub list: Vec<PropertyValue>,
    pub line_number: usize,
    pub column_number: usize,
}

impl Arguments {
    pub fn from_vec(
        arguments: Vec<PropertyValue>,
        line_number: usize,
        column_number: usize,
    ) -> Self {
        Arguments {
            list: arguments,
            line_number,
            column_number,
        }
    }

    pub fn first(&self) -> Option<&PropertyValue> {
        self.list.first()
    }

    // nth_of_type
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub name: String,
    pub data_type: rayql::types::DataType,
    pub properties: Vec<PropertyValue>,
    pub line_number: usize,
    pub column_number: usize,
}

impl Field {
    pub fn new(
        name: String,
        data_type: rayql::types::DataType,
        properties: Vec<PropertyValue>,
        line_number: usize,
        column_number: usize,
    ) -> Self {
        Field {
            name,
            data_type,
            properties,
            line_number,
            column_number,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
    pub line_number: usize,
    pub column_number: usize,
}

impl Model {
    pub fn new(name: String, fields: Vec<Field>, line_number: usize, column_number: usize) -> Self {
        Model {
            name,
            fields,
            line_number,
            column_number,
        }
    }
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
