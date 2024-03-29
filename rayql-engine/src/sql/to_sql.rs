use rayql::{
    schema::{
        Argument, ArgumentValue, Arguments, Enum, EnumVariant, FunctionCall, Model, Property,
        Reference, Schema,
    },
    sql::error::{FunctionError, ToSQLError},
    types::DataType,
    Value,
};

impl Schema {
    pub fn to_sql(&self) -> Result<Vec<String>, ToSQLError> {
        let mut sql_statements = Vec::new();

        for model in &self.models {
            let mut fields_sql = Vec::new();
            let mut fk_sql = Vec::new();

            for field in &model.fields {
                let mut field_sql = format!("    {} {}", field.name, field.data_type.to_sql(true));

                if let DataType::Enum(enum_name) = &field.data_type {
                    let variants: Vec<String> = match self.get_enum(enum_name) {
                        Some(e) => e
                            .variants
                            .iter()
                            .map(|variant| format!("'{}'", variant.to_sql()))
                            .collect(),
                        None => {
                            return Err(ToSQLError::EnumNotFound {
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
                    match prop {
                        Property::FunctionCall(FunctionCall {
                            name,
                            context,
                            arguments,
                            ..
                        }) if name.eq("foreign_key") => {
                            fk_sql.push(rayql::sql::function::foreign_key(
                                self,
                                arguments,
                                context.property_name.clone(),
                            )?);
                        }
                        _ => field_sql.push_str(&format!(" {}", prop.to_sql(self)?)),
                    }
                }

                fields_sql.push(field_sql);
            }
            fields_sql.extend(fk_sql);
            let model_sql = format!(
                "CREATE TABLE IF NOT EXISTS {} (\n{}\n);",
                model.name,
                fields_sql.join(",\n")
            );
            sql_statements.push(model_sql);
        }

        Ok(sql_statements)
    }
}

impl Property {
    pub fn to_sql(&self, schema: &Schema) -> Result<String, ToSQLError> {
        match &self {
            Property::PrimaryKey => Ok("PRIMARY KEY".to_string()),
            Property::AutoIncrement => Ok("AUTOINCREMENT".to_string()),
            Property::Unique => Ok("UNIQUE".to_string()),
            Property::FunctionCall(func) => func.to_sql(schema),
        }
    }
}

impl Reference {
    pub fn field_reference_to_sql(&self, schema: &Schema) -> Result<String, ToSQLError> {
        match schema.get_model(&self.entity) {
            Some(model) => model.field_to_sql(&self.property, self.line_number, self.column),
            None => Err(ToSQLError::ModelNotFound {
                model_name: self.entity.clone(),
                line_number: self.line_number,
                column: self.column,
            }),
        }
    }

    pub fn variant_reference_to_sql(&self, schema: &Schema) -> Result<String, ToSQLError> {
        match schema.get_enum(&self.entity) {
            Some(e) => e.variant_to_sql(&self.property, self.line_number, self.column),
            None => Err(ToSQLError::EnumNotFound {
                enum_name: self.entity.clone(),
                line_number: self.line_number,
                column: self.column,
            }),
        }
    }
}

impl Model {
    pub fn field_to_sql(
        &self,
        field_name: &str,
        line_number: usize,
        column: usize,
    ) -> Result<String, ToSQLError> {
        match self.get_field(field_name) {
            Some(_) => Ok(format!("{}({})", self.name, field_name)),
            None => Err(ToSQLError::FieldNotFound {
                field_name: field_name.to_string(),
                model_name: self.name.to_string(),
                line_number,
                column: column + field_name.len() + 1,
            }),
        }
    }
}

impl Enum {
    pub fn variant_to_sql(
        &self,
        variant: &str,
        line_number: usize,
        column: usize,
    ) -> Result<String, ToSQLError> {
        match self.get_variant(variant) {
            Some(_) => Ok(format!("'{}'", variant)),
            None => Err(ToSQLError::VariantNotFound {
                variant: variant.to_string(),
                enum_name: self.name.to_string(),
                line_number,
                column: column + variant.len() + 1,
            }),
        }
    }
}

impl EnumVariant {
    pub fn to_sql(&self) -> String {
        self.name.to_string()
    }
}

impl FunctionCall {
    pub fn to_sql(&self, schema: &Schema) -> Result<String, ToSQLError> {
        match self.name.as_str() {
            "now" => Ok("CURRENT_TIMESTAMP".to_string()),
            "min" => rayql::sql::function::min(schema, &self.context, &self.arguments),
            "max" => rayql::sql::function::max(schema, &self.context, &self.arguments),
            "references" => rayql::sql::function::references(schema, &self.arguments),
            "default" => rayql::sql::function::default(schema, &self.arguments),
            _ => Err(ToSQLError::FunctionError {
                source: FunctionError::UndefinedFunction(self.name.clone()),
                line_number: self.line_number,
                column: self.column,
            }),
        }
    }
}

impl Arguments {
    pub fn to_sql(&self, schema: &Schema) -> Result<Vec<String>, ToSQLError> {
        self.list.iter().map(|arg| arg.to_sql(schema)).collect()
    }
}

impl Argument {
    pub fn to_sql(&self, schema: &Schema) -> Result<String, ToSQLError> {
        self.value.to_sql(schema)
    }
}

impl ArgumentValue {
    pub fn to_sql(&self, schema: &Schema) -> Result<String, ToSQLError> {
        match self {
            ArgumentValue::Identifier(identifier) => Ok(identifier.clone()),
            ArgumentValue::FunctionCall(func) => func.to_sql(schema),
            ArgumentValue::Value(value) => Ok(value.to_sql()),
            _ => unimplemented!(),
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
    pub fn to_sql(&self, not_null: bool) -> String {
        let null_suffix = if not_null { "NOT NULL" } else { "NULL" };
        let data_type = match &self {
            DataType::String | DataType::Enum(_) => "TEXT",
            DataType::Integer => "INTEGER",
            DataType::Real => "REAL",
            DataType::Blob => "BLOB",
            DataType::Boolean => "BOOLEAN",
            DataType::Timestamp => "TIMESTAMP",
            DataType::Optional(inner_type) => return inner_type.to_sql(false),
        };

        format!("{} {}", data_type, null_suffix)
    }
}
