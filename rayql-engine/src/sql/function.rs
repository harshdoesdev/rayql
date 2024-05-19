use rayql::{
    schema::{ArgumentValue, Arguments, FunctionCallContext, Schema},
    sql::error::{FunctionError, ToSQLError},
    types::DataType,
};

use rayql::sql::fn_helpers::{check_value, get_single_argument};

check_value_fn!(min, "<=");

check_value_fn!(max, ">=");

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
        ArgumentValue::Value(value) if value.get_type().eq(&context.property_data_type.data_type) => {
            Ok(value.to_sql())
        },
        ArgumentValue::FunctionCall(func) => func.to_sql(schema),
        ArgumentValue::Reference(reference) => {
            if let DataType::Enum(ref enum_name) = context.property_data_type.data_type {
                if reference.entity.ne(enum_name) {
                    return Err(ToSQLError::IncorrectReference { entity_name: reference.entity, variant_name: reference.property, given_entity_name: enum_name.clone(), line_number: reference.line_number, column: reference.column, });
                }
            }

            reference.variant_reference_to_sql(schema)
        },
    )?;

    Ok(format!("DEFAULT {}", value))
});
