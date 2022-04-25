use crate::{generators::InsertOrUpdateGenerator, reflection::StructProperty};

pub fn fn_insert_or_update(result: &mut String, fields: &[StructProperty]) {
    let mut insert_or_update = InsertOrUpdateGenerator::new();

    for property in fields {
        if property.ty.is_date_time() {
            insert_or_update
                .add_insert_field_with_raw_value(property.get_db_field_name(), "DateTime");
        } else {
            insert_or_update
                .add_insert_field_value(property.get_db_field_name(), property.name.as_str());
        }
    }

    result.push_str("let sql = format!(\")");

    insert_or_update.generate_sql(result, "{__table_name}", "{__pk_name}");

    result.push_str("\", __table_name = table_name");
    result.push_str(");\n");

    result.push_str("client.execute(sql.as_str(),");
    result.push_str("&[");
    crate::postgres_utils::generate_fields_as_params(result, fields.iter());
    result.push_str("],).await?;\n");

    result.push_str("Ok(())");
}
