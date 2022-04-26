use crate::reflection::StructProperty;

pub fn impl_bulk_insert(result: &mut String, fields: &[StructProperty]) {
    result.push_str("let mut sql = my_postgres_utils::BulkInsertBuilder::new();");

    for property in fields {
        result
            .push_str(format!("sql.append_field(\"{}\");", property.get_db_field_name()).as_str());
    }

    result.push_str("for db_entity in entities {");
    result.push_str("sql.start_new_value_line();");

    for property in fields {
        if property.ty.is_date_time() {
            result.push_str(
                format!(
                    "sql.append_value_raw(&db_entity.{}.to_rfc3339());",
                    property.name
                )
                .as_str(),
            );
        } else {
            result.push_str(
                format!(
                    "sql.append_value(&db_entity.{});",
                    property.get_db_field_name()
                )
                .as_str(),
            );
        }
    }
    result.push_str("}");

    crate::postgres_utils::generate_runtime_execution(result, fields);
}
