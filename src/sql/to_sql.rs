use rayql::{
    schema::{FunctionCall, PropertyValue, Schema},
    types::DataType,
    Value,
};

impl Schema {
    pub fn to_sql(&self) -> Vec<(String, String)> {
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
                                .map(|variant| format!("'{}'", variant))
                                .collect(),
                            None => panic!("Enum not found: {}", enum_name),
                        };
                    field_sql.push_str(&format!(
                        " CHECK({} IN ({}))",
                        field.name,
                        variants.join(", ")
                    ));
                }

                for prop in &field.properties {
                    field_sql.push_str(&format!(" {}", prop.to_sql()));
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

        sql_statements
    }
}

impl PropertyValue {
    pub fn to_sql(&self) -> String {
        match &self {
            PropertyValue::PrimaryKey => "PRIMARY KEY".to_string(),
            PropertyValue::AutoIncrement => "AUTOINCREMENT".to_string(),
            PropertyValue::Unique => "UNIQUE".to_string(),
            PropertyValue::Identifier(id) => id.clone(),
            PropertyValue::FunctionCall(func) => func.to_sql(),
            PropertyValue::Value(value) => value.to_sql(),
        }
    }
}

impl FunctionCall {
    pub fn to_sql(&self) -> String {
        match self.name.as_str() {
            "now" => "CURRENT_TIMESTAMP".to_string(),
            "min" => {
                let min_value = match self.arguments.get(0) {
                    Some(value) => match value {
                        PropertyValue::Value(value) => value.to_sql(),
                        PropertyValue::FunctionCall(func) => func.to_sql(),
                        _ => {
                            panic!("min value must be a value, got {:?}", value)
                        }
                    },
                    None => panic!("min accepts exactly 1 value."),
                };

                format!("CHECK({} >= {})", &self.property_name, min_value)
            }
            "foreign_key" => {
                let (reference_table, reference_key) = match self.arguments.get(0) {
                    Some(value) => match value {
                        PropertyValue::Identifier(identifier) => match identifier.split_once('.') {
                            Some(v) => v,
                            None => panic!("Reference key not found."),
                        },
                        _ => panic!("foreign key value must be an identifer"),
                    },
                    None => panic!("foreign_key accepts exactly 1 value."),
                };

                format!("REFERENCES {}({})", reference_table, reference_key)
            }
            "default" => {
                let value = match self.arguments.get(0) {
                    Some(value) => match value {
                        PropertyValue::Value(value) => value.to_sql(),
                        PropertyValue::FunctionCall(func) => func.to_sql(),
                        _ => {
                            panic!("default value must be a value, got {:?}", value)
                        }
                    },
                    None => panic!("default accepts exactly 1 value."),
                };

                format!("DEFAULT {}", value,)
            }
            _ => {
                let args_sql: Vec<String> = self.arguments.iter().map(|arg| arg.to_sql()).collect();
                format!("{}({})", self.name, args_sql.join(", "))
            }
        }
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
