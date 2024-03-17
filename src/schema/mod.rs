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

    pub fn parse(input: &str) -> Result<rayql::Schema, rayql::schema::ParseError> {
        rayql::schema::parse(input)
    }

    pub fn get_model(&self, model_name: &str) -> Option<&Model> {
        self.models.iter().find(|model| model.name.eq(model_name))
    }

    pub fn get_enum(&self, enum_name: &str) -> Option<&Enum> {
        self.enums.iter().find(|e| e.name.eq(enum_name))
    }

    // pub fn resolve_model_reference(&self, reference: &str) -> Option<()> {
    //     let (model_name, )
    //     match reference.split_once('.') {
    //         Some((model_name, field_name)) => {
    //             let model = match schema.get_model(model_name) {
    //                 Some(model) => model,
    //                 None => {
    //                     return Err(ToSQLError::ModelNotFound {
    //                         model_name: model_name.to_string(),
    //                         line_number: *line_number,
    //                         column: *column,
    //                     })
    //                 }
    //             };

    //             match model.get_field(field_name) {
    //                 Some(_) => (model_name, field_name),
    //                 None => {
    //                     return Err(ToSQLError::FieldNotFound {
    //                         field_name: field_name.to_string(),
    //                         model_name: model_name.to_string(),
    //                         line_number: *line_number,
    //                         column: column + field_name.len() + 1, // field name length + .
    //                     });
    //                 }
    //             }
    //         }
    //         None => {
    //             return Err(ToSQLError::FunctionError {
    //                 source: FunctionError::InvalidArgument(
    //                     "Reference key not found.".to_string(),
    //                 ),
    //                 line_number: *line_number,
    //                 column: *column,
    //             })
    //         }
    //     }

    //     Some(())
    // }
}
