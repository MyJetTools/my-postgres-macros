use crate::reflection::StructProperty;

pub fn fn_insert(result: &mut String, fields: &[StructProperty]) {
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

fn generate_fields_to_insert(result: &mut String, fields: &[StructProperty]) {
    result.push_str("(");

    crate::postgres_utils::generate_field_names(result, fields.iter());

    result.push_str(") VALUES (");

    crate::postgres_utils::generate_field_values(result, fields.iter());

    result.push_str(")");
}
