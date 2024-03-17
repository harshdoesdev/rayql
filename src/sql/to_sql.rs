use rayql::{
    schema::{Argument, Arguments, EnumVariant, FunctionCall, PropertyValue, Reference, Schema},
    types::DataType,
    Value,
};

impl Schema {
    pub fn to_sql(&self) -> Result<Vec<(String, String)>, rayql::sql::ToSQLError> {
        let mut sql_statements = Vec::new();

        for model in &self.models {
            let mut fields_sql = Vec::new();
            for field in &model.fields {
                let mut field_sql = format!("    {} {}", field.name, field.data_type.to_sql(),);

                if let DataType::Enum(enum_name) = &field.data_type {
                    let variants: Vec<String> = match self.get_enum(enum_name) {
                        Some(e) => e
                            .variants
                            .iter()
                            .map(|variant| format!("'{}'", variant.to_sql()))
                            .collect(),
                        None => {
                            return Err(rayql::sql::ToSQLError::EnumNotFound {
                                enum_name: enum_name.clone(),
                                line_number: field.line_number,
                                column: field.column,
                            })
                        }
                    };
                    field_sql.push_str(&format!(
                        " CHECK({} IN ({}))",
                        field.name,
                        variants.join(", ")
                    ));
                }

                for prop in &field.properties {
                    field_sql.push_str(&format!(" {}", prop.to_sql(&self)?));
                }

                fields_sql.push(field_sql);
            }
            let model_sql = format!(
                "CREATE TABLE {} (\n{}\n);",
                model.name,
                fields_sql.join(",\n")
            );
            sql_statements.push((model.name.clone(), model_sql));
        }

        Ok(sql_statements)
    }
}

impl PropertyValue {
    pub fn to_sql(&self, schema: &Schema) -> Result<String, rayql::sql::ToSQLError> {
        match &self {
            PropertyValue::PrimaryKey => Ok("PRIMARY KEY".to_string()),
            PropertyValue::AutoIncrement => Ok("AUTOINCREMENT".to_string()),
            PropertyValue::Unique => Ok("UNIQUE".to_string()),
            PropertyValue::Identifier(id) => Ok(id.clone()),
            PropertyValue::Reference(reference) => reference.to_sql(schema),
            PropertyValue::FunctionCall(func) => func.to_sql(schema),
            PropertyValue::Value(value) => Ok(value.to_sql()),
        }
    }
}

impl Reference {
    pub fn to_sql(&self, schema: &Schema) -> Result<String, rayql::sql::ToSQLError> {
        let model = match schema.get_model(&self.entity) {
            Some(model) => model,
            None => {
                return Err(rayql::sql::ToSQLError::ModelNotFound {
                    model_name: self.entity.to_string(),
                    line_number: self.line_number.clone(),
                    column: self.column.clone(),
                })
            }
        };

        match model.get_field(&self.property) {
            Some(_) => Ok(format!("{}({})", self.entity, self.property)),
            None => {
                return Err(rayql::sql::ToSQLError::FieldNotFound {
                    field_name: self.entity.to_string(),
                    model_name: self.property.to_string(),
                    line_number: self.line_number,
                    column: self.column + self.property.len() + 1,
                });
            }
        }
    }
}

impl FunctionCall {
    pub fn to_sql(&self, schema: &Schema) -> Result<String, rayql::sql::ToSQLError> {
        match self.name.as_str() {
            "now" => Ok("CURRENT_TIMESTAMP".to_string()),
            "min" => {
                rayql::sql::functions::min_function(schema, &self.property_name, &self.arguments)
            }
            "max" => {
                rayql::sql::functions::max_function(schema, &self.property_name, &self.arguments)
            }
            "foreign_key" => rayql::sql::functions::foreign_key(schema, &self.arguments),
            "default" => rayql::sql::functions::default_fn(schema, &self.arguments),
            _ => Err(rayql::sql::ToSQLError::FunctionError {
                source: rayql::sql::FunctionError::UndefinedFunction(self.name.clone()),
                line_number: self.line_number,
                column: self.column,
            }),
        }
    }
}

impl Arguments {
    pub fn to_sql(&self, schema: &Schema) -> Result<Vec<String>, rayql::sql::ToSQLError> {
        self.list.iter().map(|arg| arg.to_sql(schema)).collect()
    }
}

impl Argument {
    pub fn to_sql(&self, schema: &Schema) -> Result<String, rayql::sql::ToSQLError> {
        self.value.to_sql(schema)
    }
}

impl EnumVariant {
    pub fn to_sql(&self) -> String {
        self.name.to_string()
    }
}

impl Value {
    pub fn to_sql(&self) -> String {
        match self {
            Value::StringLiteral(s) => format!("'{}'", s),
            Value::Integer(i) => i.to_string(),
            Value::Real(f) => {
                if *f == 0.0 {
                    "0.0".to_string()
                } else {
                    format!("{:.}", f)
                }
            }
            Value::Boolean(b) => {
                if *b {
                    "1".to_string()
                } else {
                    "0".to_string()
                }
            }
        }
    }
}

impl DataType {
    pub fn to_sql(&self) -> String {
        match &self {
            DataType::String | DataType::Enum(_) => "TEXT".to_string(),
            DataType::Integer => "INTEGER".to_string(),
            DataType::Real => "REAL".to_string(),
            DataType::Blob => "BLOB".to_string(),
            DataType::Boolean => "BOOLEAN".to_string(),
            DataType::Timestamp => "TIMESTAMP".to_string(),
            DataType::Optional(inner_type) => format!("{} NULL", inner_type.to_sql()),
        }
    }
}
