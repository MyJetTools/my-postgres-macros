use crate::reflection::StructProperty;

pub fn fn_bulk_insert_or_update(result: &mut String, fields: &[StructProperty]) {
    result.push_str("let builder = client.build_transaction();");

    result.push_str("let transaction = builder.start().await?;");

    result.push_str("for entity in entities {");

    result.push_str(
        "let mut sql_builder = my_postgres_utils::insert_or_update::InsertOrUpdateBuilder::new();",
    );

    for property in fields {
        crate::postgres_utils::read_value(result, property);
        result.push_str(" sql_builder.add_field(\"");
        result.push_str(property.get_db_field_name());
        result.push_str("\", sql_value, ");

        if property.is_primary_key() {
            result.push_str("true);")
        } else {
            result.push_str("false);")
        }
    }

    result.push_str("let sql = sql_builder.build(table_name, pk_name);");

    result.push_str("transaction.execute(sql.as_str(), sql_builder.get_values_data()).await?;");

    result.push_str("}");

    result.push_str("transaction.commit().await?;\n");

    result.push_str("Ok(())");
}
