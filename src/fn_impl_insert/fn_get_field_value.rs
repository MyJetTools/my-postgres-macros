use types_reader::StructProperty;

pub fn fn_get_field_value(result: &mut String, fields: &[StructProperty]) {
    result.push_str("match no{");
    for (i, field) in fields.iter().enumerate() {
        result.push_str(i.to_string().as_str());
        result.push_str("=> my_postgres:: SqlValue::Value(");
        crate::get_field_value::get_field_value(result, field);
        result.push_str("),");
    }
    result.push_str("_=>panic!(\"no such field with number {}\", no)");
    result.push_str("}");
}
