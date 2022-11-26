use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_from(result: &mut String, fields: &[StructProperty]) {
    result.push_str("Self{");

    for field in fields {
        result.push_str(field.name.as_str());
        result.push_str(": ");
        result.push_str(field.ty.as_str().as_str());
        result.push_str("::from_db_row(db_row, \"");
        result.push_str(field.get_db_field_name());
        result.push_str("\",");
        super::fill_sql_type(result, field);
        result.push_str("),");
    }

    result.push_str("}");
}
