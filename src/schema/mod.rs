mod parser;
mod tokenizer;
mod utils;

pub mod error;

pub use parser::parse;

#[derive(Debug, PartialEq, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub line_number: usize,
    pub column: usize,
}

impl Enum {
    pub fn new(
        name: String,
        variants: Vec<EnumVariant>,
        line_number: usize,
        column: usize,
    ) -> Self {
        Enum {
            name,
            variants,
            line_number,
            column,
        }
    }

    pub fn get_variant(&self, enum_variant: &str) -> Option<&EnumVariant> {
        self.variants
            .iter()
            .find(|variant| variant.name.eq(enum_variant))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub line_number: usize,
    pub column: usize,
}

impl EnumVariant {
    pub fn new(name: String, line_number: usize, column: usize) -> Self {
        EnumVariant {
            name,
            line_number,
            column,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyValue {
    Identifier(String),
    Reference(Reference),
    FunctionCall(FunctionCall),
    Value(rayql::value::Value),
    PrimaryKey,
    AutoIncrement,
    Unique,
    Required,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Reference {
    pub entity: String,
    pub property: String,
    pub line_number: usize,
    pub column: usize,
}

impl Reference {
    pub fn new(entity: String, property: String, line_number: usize, column: usize) -> Self {
        Reference {
            entity,
            property,
            line_number,
            column,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Arguments,
    pub property_name: String,
    pub line_number: usize,
    pub column: usize,
}

impl FunctionCall {
    pub fn new(
        property_name: String,
        name: String,
        arguments: Vec<Argument>,
        line_number: usize,
        column: usize,
    ) -> Self {
        FunctionCall {
            name,
            arguments: Arguments::from_vec(arguments, line_number, column),
            property_name,
            line_number,
            column,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Arguments {
    pub list: Vec<Argument>,
    pub line_number: usize,
    pub column: usize,
}

impl Arguments {
    pub fn from_vec(arguments: Vec<Argument>, line_number: usize, column: usize) -> Self {
        Arguments {
            list: arguments,
            line_number,
            column,
        }
    }

    pub fn first(&self) -> Option<&Argument> {
        self.list.first()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Argument {
    pub value: PropertyValue,
    pub line_number: usize,
    pub column: usize,
}

impl Argument {
    pub fn new(value: PropertyValue, line_number: usize, column: usize) -> Self {
        Argument {
            value,
            line_number,
            column,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub name: String,
    pub data_type: rayql::types::DataType,
    pub properties: Vec<PropertyValue>,
    pub line_number: usize,
    pub column: usize,
}

impl Field {
    pub fn new(
        name: String,
        data_type: rayql::types::DataType,
        properties: Vec<PropertyValue>,
        line_number: usize,
        column: usize,
    ) -> Self {
        Field {
            name,
            data_type,
            properties,
            line_number,
            column,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
    pub line_number: usize,
    pub column: usize,
}

impl Model {
    pub fn new(name: String, fields: Vec<Field>, line_number: usize, column: usize) -> Self {
        Model {
            name,
            fields,
            line_number,
            column,
        }
    }

    pub fn get_field(&self, field_name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name.eq(field_name))
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

    pub fn parse(input: &str) -> Result<rayql::Schema, error::ParseError> {
        rayql::schema::parse(input)
    }

    pub fn get_model(&self, model_name: &str) -> Option<&Model> {
        self.models.iter().find(|model| model.name.eq(model_name))
    }

    pub fn get_enum(&self, enum_name: &str) -> Option<&Enum> {
        self.enums.iter().find(|e| e.name.eq(enum_name))
    }
}
