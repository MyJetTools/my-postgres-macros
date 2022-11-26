use types_reader::StructProperty;

pub fn fn_get_order_by_fields(result: &mut String, fields: &[StructProperty]) {
    let mut order_by_desc = Vec::with_capacity(fields.len());
    let mut order_by = Vec::with_capacity(fields.len());

    for prop in fields {
        if prop.attrs.has_attr("order_by_desc") {
            order_by_desc.push(prop.name.as_str());
            continue;
        }

        if prop.attrs.has_attr("order_by") {
            order_by.push(prop.name.as_str());
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

    if !order_by_desc.is_empty() {
        result.push_str("Some(my_postgres::sql_select::OrderByFields::Desc(vec![");
        for field in order_by_desc {
            result.push_str("\"");
            result.push_str(field);
            result.push_str("\",");
        }
        result.push_str("]))");
    }

    if !order_by.is_empty() {
        result.push_str("Some(my_postgres::sql_select::OrderByFields::Asc(vec![");
        for field in order_by {
            result.push_str("\"");
            result.push_str(field);
            result.push_str("\",");
        }
        result.push_str("]))");
    }
}
