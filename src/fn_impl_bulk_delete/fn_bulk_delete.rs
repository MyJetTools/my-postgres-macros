use crate::reflection::StructProperty;

pub fn fn_bulk_delete(result: &mut String, fields: &[StructProperty]) {
    result.push_str("let mut sql_builder = my_postgres_utils::delete::BulkDeleteBuilder::new();");

    result.push_str("for entity in entities {");
    result.push_str("sql_builder.add_new_line();");
    for property in fields {
        if property.is_primary_key() {
            crate::postgres_utils::read_value(result, property);

            result.push_str("sql_builder.add_where_field(\"");
            result.push_str(property.get_db_field_name());
            result.push_str("\", sql_value);");
        }
    }

    result.push_str("}");
    result.push_str("let sql = sql_builder.build(table_name);");

    result.push_str("client.execute(sql.as_str(), sql_builder.get_values_data()).await?;\n");

    result.push_str("Ok(())");
}
