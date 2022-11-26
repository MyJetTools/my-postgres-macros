use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_fill_order_by(result: &mut String, fields: &[StructProperty]) {
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
        return;
    }

    if !order_by_desc.is_empty() && !order_by.is_empty() {
        panic!("Ether order_by_desc or order_by must be set, not both");
    }

    let mut order_by_content = String::new();
    order_by_content.push_str(" ORDER BY");

    if !order_by_desc.is_empty() {
        for field in order_by_desc {
            order_by_content.push(' ');
            order_by_content.push_str(field.get_db_field_name());
        }
        order_by_content.push_str(" DESC");
        result.push_str(order_by_content.as_str());
        return;
    }

    if !order_by.is_empty() {
        for field in order_by {
            order_by_content.push(' ');
            order_by_content.push_str(field.get_db_field_name());
        }

        result.push_str(order_by_content.as_str());
    }
}
