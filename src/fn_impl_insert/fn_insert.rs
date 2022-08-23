use crate::reflection::StructProperty;

pub fn fn_insert(result: &mut String, fields: &[StructProperty]) {
    for property in fields {
        if property.has_ignore_if_null_attr() || property.ty.is_option() {
            if property.ty.is_option_of_string() {
                result.push_str("if let Some(sql_value) = &self.");
            } else {
                result.push_str("if let Some(sql_value) = self.");
            }

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
