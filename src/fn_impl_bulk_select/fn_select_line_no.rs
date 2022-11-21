use types_reader::StructProperty;

pub fn fn_select_line_no(result: &mut String, struct_properties: &[StructProperty]) {
    for struct_property in struct_properties {
        if struct_property.attrs.has_attr("line_no") || struct_property.name == "line_no" {
            result.push_str("self.");
            result.push_str(struct_property.name.as_str());
            return;
        }
    }

    panic!("line_no not found");
}
