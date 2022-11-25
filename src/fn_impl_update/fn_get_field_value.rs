use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_get_field_value(result: &mut String, fields: &[StructProperty]) {
    result.push_str("match no{");

    let mut i = 0;
    for field in fields {
        if field.is_primary_key() {
            continue;
        }

        result.push_str(i.to_string().as_str());
        result.push_str("=> my_postgres::sql_update::SqlUpdateValue{ name: \"");
        result.push_str(field.get_db_field_name());

        result.push_str("\", value: ");

        crate::get_field_value::get_field_value(result, field);
        result.push_str("},");

        i += 1;
    }
    result.push_str("_=>panic!(\"no such field with number {}\", no)");
    result.push_str("}");
}
