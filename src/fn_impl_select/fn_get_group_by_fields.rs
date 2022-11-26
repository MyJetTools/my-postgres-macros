use types_reader::StructProperty;

pub fn fn_get_group_by_fields(result: &mut String, fields: &[StructProperty]) {
    let mut group_by = Vec::with_capacity(fields.len());

    for prop in fields {
        if prop.attrs.has_attr("order_by") {
            group_by.push(prop.name.as_str());
            continue;
        }
    }

    if group_by.is_empty() {
        result.push_str("None");
        return;
    }

    result.push_str("Some(my_postgres::sql_select::GroupByFields{fields: vec![");
    for field in group_by {
        result.push_str("\"");
        result.push_str(field);
        result.push_str("\",");
    }
    result.push_str("]})");
}
