use types_reader::StructProperty;

pub fn fn_get_param_value(result: &mut String, struct_properties: &[StructProperty]) {
    let mut no = 1;

    result.push_str("match no {");
    for struct_property in struct_properties {
        result.push_str(no.to_string().as_str());
        result.push_str("=> &self.");
        result.push_str(&struct_property.name);
        result.push(',');
        no += 1;
    }

    result.push_str("_ => panic!(\"Unexpected param no {}\", no)}");
}
