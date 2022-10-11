use crate::{
    postgres_utils::ReadingSoruce,
    reflection::{PropertyType, StructProperty},
};

pub fn fn_insert(result: &mut String, fields: &[StructProperty]) {
    for property in fields {
        if let PropertyType::OptionOf(sub_type) = &property.ty {
            if sub_type.is_string() {
                result.push_str("if let Some(sql_value) = &self.");
            } else {
                result.push_str("if let Some(sql_value) = self.");
            }

            result.push_str(&property.name);
            result.push_str("{");

            if sub_type.is_string() {
                crate::postgres_utils::read_value(
                    result,
                    sub_type,
                    ReadingSoruce::Variable("sql_value"),
                );
            }

            result.push_str("sql_builder.append_field(\"");
            result.push_str(&property.name);
            result.push_str("\", sql_value);\n ");
            result.push_str("}");
        } else {
            if property.ty.is_string() {
                crate::postgres_utils::read_value(
                    result,
                    &property.ty,
                    ReadingSoruce::ItSelfAsStr(&property.name),
                );
            } else {
                crate::postgres_utils::read_value(
                    result,
                    &property.ty,
                    ReadingSoruce::ItSelf(&property.name),
                );
            }
            result.push_str("sql_builder.append_field(\"");
            result.push_str(&property.name);
            result.push_str("\", sql_value);\n ");
        }
    }
}
