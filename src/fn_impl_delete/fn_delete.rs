use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_delete(result: &mut String, fields: &[StructProperty]) {
    for property in fields {
        if property.is_primary_key() {
            crate::postgres_utils::read_value(result, property, None);

            result.push_str("sql_builder.add_where_field(\"");
            result.push_str(property.get_db_field_name());
            result.push_str("\", sql_value);");
        }
    }
}
