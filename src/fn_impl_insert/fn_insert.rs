use crate::reflection::StructProperty;

pub fn fn_insert(result: &mut String, fields: &[StructProperty]) {
    for property in fields {
        if property.has_ignore_if_null_attr() {
            result.push_str("if let Some(value) = &self.");
            result.push_str(&property.name);
            result.push_str("{");
            crate::postgres_utils::read_value(result, property, "value");
            result.push_str("sql_builder.append_field(\"");
            result.push_str(&property.name);
            result.push_str("\", sql_value);\n ");
            result.push_str("}");
        } else {
            crate::postgres_utils::read_value(result, property, "self");
            result.push_str("sql_builder.append_field(\"");
            result.push_str(&property.name);
            result.push_str("\", sql_value);\n ");
        }
    }
}
