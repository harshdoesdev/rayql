#[derive(Debug, PartialEq, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub name: String,
    pub data_type: rayql::types::DataType,
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
