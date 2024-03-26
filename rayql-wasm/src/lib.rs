use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_sql(schema_src: &str) -> Result<JsValue, JsValue> {
    let schema = match rayql_engine::Schema::parse(schema_src) {
        Ok(schema) => schema,
        Err(error) => {
            return Err(JsValue::from_str(&format!(
                "Error parsing schema: {}",
                error
            )))
        }
    };

    let sql = match schema.to_sql() {
        Ok(sql) => sql,
        Err(error) => {
            return Err(JsValue::from_str(&format!(
                "Error converting schema to SQL: {}",
                error
            )))
        }
    };

    let sql_str = sql.join("\n\n");
    Ok(JsValue::from_str(&sql_str))
}
