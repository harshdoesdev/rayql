use rayql::{
    schema::{Argument, Arguments, EnumVariant, FunctionCall, PropertyValue, Schema},
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
                    let variants: Vec<String> =
                        match self.enums.iter().find(|e| e.name.eq(enum_name)) {
                            Some(e) => e
                                .variants
                                .iter()
                                .map(|variant| format!("'{}'", variant.to_sql()))
                                .collect(),
                            None => {
                                return Err(rayql::sql::ToSQLError::EnumNotFound {
                                    enum_name: enum_name.clone(),
                                    line_number: field.line_number,
                                    column_number: field.column_number,
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
                    field_sql.push_str(&format!(" {}", prop.to_sql()?));
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
    pub fn to_sql(&self) -> Result<String, rayql::sql::ToSQLError> {
        match &self {
            PropertyValue::PrimaryKey => Ok("PRIMARY KEY".to_string()),
            PropertyValue::AutoIncrement => Ok("AUTOINCREMENT".to_string()),
            PropertyValue::Unique => Ok("UNIQUE".to_string()),
            PropertyValue::Identifier(id) => Ok(id.clone()),
            PropertyValue::FunctionCall(func) => func.to_sql(),
            PropertyValue::Value(value) => Ok(value.to_sql()),
        }
    }
}

impl FunctionCall {
    pub fn to_sql(&self) -> Result<String, rayql::sql::ToSQLError> {
        match self.name.as_str() {
            "now" => Ok("CURRENT_TIMESTAMP".to_string()),
            "min" => {
                rayql::sql::function_to_sql::min_function(&self.property_name, &self.arguments)
            }
            "foreign_key" => rayql::sql::function_to_sql::foreign_key(&self.arguments),
            "default" => rayql::sql::function_to_sql::default_fn(&self.arguments),
            _ => Err(rayql::sql::ToSQLError::FunctionError {
                source: rayql::sql::FunctionError::UndefinedFunction(self.name.clone()),
                line_number: self.line_number,
                column_number: self.column_number,
            }),
        }
    }
}

impl Arguments {
    pub fn to_sql(&self) -> Result<Vec<String>, rayql::sql::ToSQLError> {
        self.list.iter().map(|arg| arg.to_sql()).collect()
    }
}

impl Argument {
    pub fn to_sql(&self) -> Result<String, rayql::sql::ToSQLError> {
        self.value.to_sql()
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
