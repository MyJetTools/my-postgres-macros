use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_fill_group_by(result: &mut String, fields: &[StructProperty]) {
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

    result.push_str("result.push_str(\" GROUP BY\");");
    for prop in group_by {
        result.push_str("sql.push(' ');");
        result.push_str("sql.push_str(\"");
        result.push_str(prop);
        result.push_str("\");");
    }
}
