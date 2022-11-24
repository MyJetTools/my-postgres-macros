use types_reader::{PropertyType, StructProperty};

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_get_field_value(result: &mut String, struct_properties: &[StructProperty]) {
    let mut no = 0;

    result.push_str("match no {");
    for struct_property in struct_properties {
        result.push_str(no.to_string().as_str());
        result.push_str("=> ");
        get_field_value(result, struct_property);
        result.push(',');
        no += 1;
    }

    result.push_str("_ => panic!(\"Unexpected param no {}\", no)}");
}

fn get_field_value(result: &mut String, struct_propery: &StructProperty) {
    match &struct_propery.ty {
        types_reader::PropertyType::U8 => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::I8 => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::U16 => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::I16 => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::U32 => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::I32 => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::U64 => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::I64 => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::F32 => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::F64 => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::String => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::Str => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::DateTime => fill_sql_value(result, struct_propery),
        types_reader::PropertyType::OptionOf(sub_type) => {
            fill_option_of(result, struct_propery, &sub_type)
        }
        types_reader::PropertyType::VecOf(sub_type) => {
            get_field_value_of_vec(result, struct_propery, sub_type)
        }
        _ => panic!("{} is not supported", struct_propery.ty.as_str()),
    }
}

fn fill_option_of(result: &mut String, struct_propery: &StructProperty, sub_type: &PropertyType) {
    match &sub_type {
        types_reader::PropertyType::U8 => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::I8 => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::U16 => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::I16 => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::U32 => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::I32 => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::U64 => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::I64 => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::F32 => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::F64 => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::String => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::Str => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::DateTime => fill_option_of_sql_value(result, struct_propery),
        types_reader::PropertyType::VecOf(sub_type) => {
            fill_option_of_vec(result, struct_propery, sub_type)
        }
        _ => panic!("{} is not supported", struct_propery.ty.as_str()),
    }
}

fn fill_option_of_vec(
    result: &mut String,
    struct_propery: &StructProperty,
    sub_type: &PropertyType,
) {
    match sub_type {
        types_reader::PropertyType::U8 => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::I8 => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::U16 => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::I16 => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::U32 => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::I32 => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::U64 => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::I64 => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::F32 => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::F64 => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::String => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::Str => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::DateTime => fill_option_of_vec_of_value(result, struct_propery),
        types_reader::PropertyType::Struct(_) => {
            fill_option_of_vec_of_value(result, struct_propery)
        }
        _ => panic!("Vec<{}> is not supported", sub_type.as_str()),
    }
}

fn get_field_value_of_vec(
    result: &mut String,
    struct_propery: &StructProperty,
    sub_type: &PropertyType,
) {
    match sub_type {
        types_reader::PropertyType::U8 => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::I8 => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::U16 => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::I16 => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::U32 => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::I32 => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::U64 => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::I64 => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::F32 => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::F64 => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::String => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::Str => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::DateTime => fill_vec_of_sql_value(result, struct_propery),
        types_reader::PropertyType::Struct(_) => fill_vec_of_sql_value(result, struct_propery),
        _ => panic!("Vec<{}> is not supported", sub_type.as_str()),
    }
}

fn fill_sql_value(result: &mut String, struct_propery: &StructProperty) {
    result.push_str("my_postgres::sql_where::SqlWhereValue::AsValue { name: \"");
    result.push_str(struct_propery.get_db_field_name());
    result.push_str("\", value: Some(&self.");
    result.push_str(&struct_propery.name);
    result.push_str("),");
    fill_op(result, struct_propery);
    result.push_str("}");
}

fn fill_option_of_sql_value(result: &mut String, struct_propery: &StructProperty) {
    result.push_str("my_postgres::sql_where::SqlWhereValue::AsValue { name: \"");
    result.push_str(struct_propery.get_db_field_name());
    result.push_str("\", value: if let Some(value) = &self.");
    result.push_str(&struct_propery.name);
    result.push_str("{Some(value)}else{None},");
    fill_op(result, struct_propery);
    result.push_str("}");
}

fn fill_option_of_vec_of_value(result: &mut String, struct_propery: &StructProperty) {
    result.push_str("my_postgres::sql_where::SqlWhereValue::to_in_operator(\"");
    result.push_str(struct_propery.get_db_field_name());
    result.push_str("\", &self.");
    result.push_str(&struct_propery.name);
    result.push_str(")");
}

fn fill_vec_of_sql_value(result: &mut String, struct_propery: &StructProperty) {
    result.push_str("my_postgres::sql_where::SqlWhereValue::to_in_operator(\"");
    result.push_str(struct_propery.get_db_field_name());
    result.push_str("\", &Some(self.");
    result.push_str(&struct_propery.name);
    result.push_str("))");
}

fn fill_op(result: &mut String, struct_propery: &StructProperty) {
    if let Some(op) = struct_propery.attrs.try_get("operator") {
        if let Some(content) = op.content.as_ref() {
            result.push_str("op: \"");
            result.push_str(extract_and_verify_operation(content));
            result.push_str("\"");
        }
        return;
    }

    if struct_propery.attrs.has_attr("ne") {
        result.push_str("op: \"!=\"");
        return;
    }

    result.push_str("op: \"=\"");
}

fn extract_and_verify_operation(src: &[u8]) -> &str {
    let result = extract_operation(src);

    if result == "="
        || result == "!="
        || result == "<"
        || result == "<="
        || result == ">"
        || result == ">="
        || result == "<>"
    {
        return result;
    }

    panic!("Invalid operator {}", result);
}

fn extract_operation(src: &[u8]) -> &str {
    let src = &src[1..src.len() - 1];

    for i in 0..src.len() {
        if src[i] == b'"' || src[i] == b'\'' {
            let b = src[i];

            for j in 1..src.len() {
                let pos = src.len() - j;

                if src[pos] == b {
                    let result = &src[i + 1..pos];

                    let result = std::str::from_utf8(result).unwrap();
                    return result;
                }
            }
        }
    }

    std::str::from_utf8(src).unwrap()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {
        let src = "(\">\")";

        let operator = super::extract_operation(src.as_bytes());

        assert_eq!(">", operator);
    }

    #[test]
    fn test_2() {
        let src = "(>)";

        let operator = super::extract_operation(src.as_bytes());

        assert_eq!(">", operator);
    }

    #[test]
    fn test_3() {
        let src = "('>')";

        let operator = super::extract_operation(src.as_bytes());

        assert_eq!(">", operator);
    }
}
