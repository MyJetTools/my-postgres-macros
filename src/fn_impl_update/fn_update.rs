use crate::{
    postgres_utils::ReadingSoruce,
    reflection::{PropertyType, StructProperty},
};

pub fn fn_update(result: &mut String, fields: &[StructProperty]) {
    for property in fields {
        if let PropertyType::OptionOf(sub_ty) = &property.ty {
            result.push_str("if let Some(sql_value) = self.");
            result.push_str(&property.name);

            if sub_ty.is_string() {
                result.push_str(".as_ref()");
            }

            result.push_str("{");
            crate::postgres_utils::read_value(
                result,
                &sub_ty,
                ReadingSoruce::Variable("sql_value"),
            );
            result.push_str("sql_builder.append_field(\"");
            result.push_str(&property.name);
            result.push_str("\", sql_value,");
            if property.is_primary_key() {
                result.push_str(" true);");
            } else {
                result.push_str(" false);");
            }
            result.push_str("}");
        } else {
            crate::postgres_utils::read_value(
                result,
                &property.ty,
                ReadingSoruce::ItSelf(&property.name),
            );
            result.push_str("sql_builder.append_field(\"");
            result.push_str(&property.name);
            result.push_str("\", sql_value,");
            if property.is_primary_key() {
                result.push_str(" true);");
            } else {
                result.push_str(" false);");
            }
        }
    }
}
