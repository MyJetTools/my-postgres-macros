use crate::reflection::StructProperty;

pub fn fn_insert_or_update(result: &mut String, fields: &[StructProperty]) {
    for property in fields {
        crate::postgres_utils::read_value(result, property, "self");
        result.push_str("sql_builder.add_field(\"");
        result.push_str(&property.name);
        result.push_str("\", sql_value, ");

        if property.is_primary_key() {
            result.push_str(" true);");
        } else {
            result.push_str(" false);");
        }
    }
}
