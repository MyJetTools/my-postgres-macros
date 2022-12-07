use types_reader::EnumCase;

pub fn fn_as_numbered_str(result: &mut String, enum_cases: &[EnumCase]) {
    result.push_str(" match self {");

    let mut i = 0;
    for enum_case in enum_cases {
        result.push_str("Self::");
        result.push_str(enum_case.name.as_str());

        result.push_str(" => \"");
        result.push_str(i.to_string().as_str());
        result.push('"');
        result.push(',');
        i += 1;
    }
    result.push('}');
}
