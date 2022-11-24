use types_reader::StructProperty;

pub fn fn_get_field_name(result: &mut String, fields: &[StructProperty]) {
    result.push_str("match no{");
    for (i, field) in fields.iter().enumerate() {
        result.push_str(i.to_string().as_str());
        result.push_str("=>");
        result.push_str(field.name.as_str());
        result.push_str(",");
    }
    result.push_str("_=>panic!(\"no such field with number {}\", no)");
    result.push_str("}");
}
