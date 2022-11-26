use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn get_group_by_fields(result: &mut String, fields: &[StructProperty]) {
    let mut group_by = Vec::with_capacity(fields.len());

    for prop in fields {
        if prop.attrs.has_attr("group_by") {
            group_by.push(prop.get_db_field_name());
            continue;
        }
    }

    if group_by.is_empty() {
        return;
    }

    result.push_str("\" GROUP BY");
    for prop in group_by {
        result.push(' ');
        result.push_str(prop);
    }
}
