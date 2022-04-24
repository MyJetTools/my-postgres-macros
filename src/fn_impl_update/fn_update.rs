use crate::reflection::StructProperty;

pub fn fn_update(result: &mut String, fields: &[StructProperty]) {
    if crate::postgres_utils::has_ignore_if_null_attributes(fields.iter()) {
        fn_update_with_ignore_fields(result, fields)
    } else {
        fn_update_without_ignore_fields(result, fields)
    }
}

fn fn_update_with_ignore_fields(result: &mut String, fields: &[StructProperty]) {
    result.push_str("let mut sql = String::new();\n");

    result.push_str("sql.push_str(\"UPDATE \");\n");

    result.push_str("sql.push_str(table_name);\n");

    result.push_str("sql.push_str(\" SET (\");\n");

    result.push_str("let mut first_field = false;\n");

    crate::postgres_utils::generate_field_names_with_ignore(
        result,
        fields.iter().filter(|itm| !itm.is_key()),
    );

    result.push_str("sql.push_str(\") = (\");");

    result.push_str("let mut no = 1;");

    crate::postgres_utils::generate_field_values_with_ignore(
        result,
        fields.iter().filter(|itm| !itm.is_key()),
    );

    result.push_str("sql.push_str(\") WHERE \"");

    crate::postgres_utils::generte_where_with_ignore(
        result,
        fields.iter().filter(|itm| itm.is_key()),
    );

    result.push_str(");\n");

    result.push_str("client.execute(sql.as_str(),");
    result.push_str("&[");
    crate::postgres_utils::generate_fields_as_params(
        result,
        fields.iter().filter(|itm| !itm.is_key()),
    );
    crate::postgres_utils::generate_fields_as_params(
        result,
        fields.iter().filter(|itm| itm.is_key()),
    );
    result.push_str("],).await?;\n");

    result.push_str("Ok(())");
}

fn fn_update_without_ignore_fields(result: &mut String, fields: &[StructProperty]) {
    result.push_str("let sql = format!(\"UPDATE {__table_name} SET (");

    crate::postgres_utils::generate_field_names(result, fields.iter().filter(|itm| !itm.is_key()));

    result.push_str(") = (");

    let no = crate::postgres_utils::generate_field_values(
        result,
        fields.iter().filter(|itm| !itm.is_key()),
    );

    result.push_str(") WHERE ");

    generate_where(result, fields, no);

    result.push_str("\", __table_name = table_name");

    crate::postgres_utils::generate_date_time_reading(
        result,
        fields.iter().filter(|itm| itm.is_key()),
    );
    result.push_str(");\n");

    result.push_str("client.execute(sql.as_str(),");
    result.push_str("&[");
    crate::postgres_utils::generate_fields_as_params(
        result,
        fields.iter().filter(|itm| !itm.is_key()),
    );
    crate::postgres_utils::generate_fields_as_params(
        result,
        fields.iter().filter(|itm| itm.is_key()),
    );
    result.push_str("],).await?;\n");

    result.push_str("Ok(())");
}

fn generate_where(result: &mut String, fields: &[StructProperty], mut no: i32) -> i32 {
    let mut first = true;
    for prop in fields.iter().filter(|itm| itm.is_key()) {
        if !first {
            result.push_str(" AND ");
        }

        result.push_str(prop.get_db_field_name());
        result.push_str(" = ");
        no = crate::postgres_utils::generate_set_value(result, prop, no);

        first = false;
    }

    no
}
