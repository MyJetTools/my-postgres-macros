use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_get_order_by_fields(result: &mut String, fields: &[StructProperty]) {
    let mut order_by_desc = Vec::with_capacity(fields.len());
    let mut order_by = Vec::with_capacity(fields.len());

    for prop in fields {
        if prop.attrs.has_attr("order_by_desc") {
            order_by_desc.push(prop);
            continue;
        }

        if prop.attrs.has_attr("order_by") {
            order_by.push(prop);
            continue;
        }
    }

    if order_by_desc.is_empty() && order_by.is_empty() {
        result.push_str("None");
        return;
    }

    if !order_by_desc.is_empty() && !order_by.is_empty() {
        panic!("Ether order_by_desc or order_by must be set, not both");
    }

    result.push_str("Some(\" ORDER BY");

    if !order_by_desc.is_empty() {
        for field in order_by_desc {
            result.push(' ');
            result.push_str(field.get_db_field_name());
        }
        result.push_str(" DESC\")");

        return;
    }

    if !order_by.is_empty() {
        for field in order_by {
            result.push(' ');
            result.push_str(field.get_db_field_name());
        }
        result.push_str("\")");
    }
}
