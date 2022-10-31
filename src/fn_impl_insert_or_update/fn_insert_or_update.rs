use types_reader::{PropertyType, StructProperty};

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_insert_or_update(result: &mut String, fields: &[StructProperty]) {
    for property in fields {
        if let PropertyType::OptionOf(sub_type) = &property.ty {
            if sub_type.is_string() {
                result.push_str("if let Some(sql_value) = &self.");
            } else {
                result.push_str("if let Some(sql_value) = self.");
            }

            result.push_str(&property.name);
            result.push_str("{");
        }

        crate::postgres_utils::read_value(result, property, None);
        result.push_str("sql_builder.add_field(\"");
        result.push_str(&property.name);
        result.push_str("\", sql_value, ");

        if property.is_primary_key() {
            result.push_str(" true);");
        } else {
            result.push_str(" false);");
        }

        // SCRIPT END
        if property.ty.is_option() {
            result.push_str("}");
        }
    }
}
