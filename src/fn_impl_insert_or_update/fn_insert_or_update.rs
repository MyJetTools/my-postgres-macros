use crate::{generators::InsertOrUpdateGenerator, reflection::StructProperty};

pub fn fn_insert_or_update(result: &mut String, fields: &[StructProperty]) {
    let mut insert_or_update = InsertOrUpdateGenerator::new();

    for property in fields {
        if property.ty.is_date_time() {
            insert_or_update
                .add_insert_field_with_raw_value(property.get_db_field_name(), "DateTime");

            if !property.is_key() {
                insert_or_update
                    .add_update_field_with_raw_value(property.get_db_field_name(), "DateTime");
            }
        } else {
            insert_or_update
                .add_insert_field_value(property.get_db_field_name(), property.name.as_str());

            if !property.is_key() {
                insert_or_update
                    .add_insert_field_value(property.get_db_field_name(), property.name.as_str());
            }
        }
    }

    result.push_str("let sql = format!(\")");

    insert_or_update.generate_sql(result, "{__table_name}", "{__pk_name}");

    result.push_str("\", __table_name = table_name, __pk_name = pk_name");
    result.push_str(");\n");

    result.push_str("client.execute(sql.as_str(),");
    result.push_str("&[");

    for numbered_field in insert_or_update.get_numbred_fields() {
        result.push_str("&self.");
        result.push_str(numbered_field);
        result.push(',');
    }

    result.push_str("],).await?;\n");

    result.push_str("Ok(())");
}
