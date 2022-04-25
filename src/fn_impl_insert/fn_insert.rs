use crate::reflection::StructProperty;

pub fn fn_insert(result: &mut String, fields: &[StructProperty]) {
    if crate::postgres_utils::has_ignore_if_null_attributes(fields.iter()) {
        fn_insert_with_ignore_fields(result, fields)
    } else {
        fn_insert_without_ignore_fields(result, fields)
    }
}

fn fn_insert_with_ignore_fields(result: &mut String, fields: &[StructProperty]) {
    result.push_str("let mut sql = my_postgres_utils::PosrgresInsertBuilder::new();\n");

    crate::postgres_utils::generate_field_names_runtime(
        result,
        fields.iter().filter(|itm| !itm.is_key()),
    );

    crate::postgres_utils::generate_runtime_execution(result);
}

fn fn_insert_without_ignore_fields(result: &mut String, fields: &[StructProperty]) {
    result.push_str("let sql = format!(\"INSERT INTO {__table_name} ");

    generate_fields_to_insert(result, fields);

    result.push_str("\", __table_name = table_name");

    crate::postgres_utils::generate_date_time_reading(result, fields.iter());
    result.push_str(");\n");

    result.push_str("client.execute(sql.as_str(),");
    result.push_str("&[");
    crate::postgres_utils::generate_fields_as_params(result, fields.iter());
    result.push_str("],).await?;\n");

    result.push_str("Ok(())");
}

pub fn generate_fields_to_insert(result: &mut String, fields: &[StructProperty]) {
    result.push_str("(");

    crate::postgres_utils::generate_field_names(result, fields.iter());

    result.push_str(") VALUES (");

    crate::postgres_utils::generate_field_values(result, fields.iter());

    result.push_str(")");
}
