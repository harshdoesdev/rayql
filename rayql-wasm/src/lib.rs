use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_sql(schema_src: &str) -> Result<JsValue, JsValue> {
    let schema = match rayql_engine::Schema::parse(schema_src) {
        Ok(schema) => schema,
        Err(error) => {
            return Err(JsValue::from_str(
                &rayql_engine::error::generate_error_message(&error, schema_src),
            ))
        }
    };

    let sql = match schema.to_sql() {
        Ok(sql) => sql,
        Err(error) => {
            return Err(JsValue::from_str(
                &rayql_engine::error::pretty_to_sql_error_message(error, schema_src),
            ))
        }
    };

    let sql_str = sql.join("\n\n");
    Ok(JsValue::from_str(&sql_str))
}
