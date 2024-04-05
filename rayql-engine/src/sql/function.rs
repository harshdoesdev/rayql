use rayql::{
    schema::{ArgumentValue, Arguments, FunctionCallContext, Schema},
    sql::error::{FunctionError, ToSQLError},
};

use rayql::sql::fn_helpers::{check_value, get_single_argument};

check_value_fn!(min, ">=");

check_value_fn!(max, "<=");

single_arg_fn!(foreign_key(schema, argument, context) {
    let reference = argument_matches!(
        argument,
        ArgumentValue::Reference(reference) => reference.field_reference_to_sql(schema)?
    );

    Ok(format!(
        "    FOREIGN KEY ({}) REFERENCES {}",
        &context.property_name, reference
    ))
});

single_arg_fn!(references(schema, argument, _context) {
    let reference = argument_matches!(
        argument,
        ArgumentValue::Reference(reference) => reference.field_reference_to_sql(schema)?
    );

    Ok(format!("REFERENCES {}", reference))
});

single_arg_fn!(default(schema, argument, context) {
    let value = argument_matches!(
        argument,
        ArgumentValue::Value(value) if value.get_type().eq(&context.property_data_type) => {
            Ok(value.to_sql())
        },
        ArgumentValue::FunctionCall(func) => func.to_sql(schema),
        ArgumentValue::Reference(reference) => reference.variant_reference_to_sql(schema),
    )?;

    Ok(format!("DEFAULT {}", value))
});
