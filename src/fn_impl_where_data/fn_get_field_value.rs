use types_reader::StructProperty;

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
        types_reader::PropertyType::DateTime => {
            if struct_propery.has_bigint_attr() {
                result.push_str("my_postgres::InputDataValue::AsNonString { name: \"");
                result.push_str(struct_propery.get_db_field_name());
                result.push_str("\", value: self.");
                result.push_str(&struct_propery.name);
                result.push_str(".unix_microseconds.to_string(),");
                fill_op(result, struct_propery);
                result.push_str("}");
            } else {
                result.push_str("my_postgres::InputDataValue::AsString { name: \"");
                result.push_str(struct_propery.get_db_field_name());
                result.push_str("\", value: self.");
                result.push_str(&struct_propery.name);
                result.push_str(".to_rfc3339(),");
                fill_op(result, struct_propery);
                result.push_str("}");
            }
        }
        _ => panic!("{} is not supported", struct_propery.ty.as_str()),
    }
}

fn fill_sql_value(result: &mut String, struct_propery: &StructProperty) {
    result.push_str("my_postgres::InputDataValue::AsSqlValue { name: \"");
    result.push_str(struct_propery.get_db_field_name());
    result.push_str("\", value: &self.");
    result.push_str(&struct_propery.name);
    result.push_str(",");
    fill_op(result, struct_propery);
    result.push_str("}");
}

fn fill_op(result: &mut String, struct_propery: &StructProperty) {
    if let Some(op) = struct_propery.attrs.try_get("operator") {
        if let Some(content) = op.content.as_ref() {
            result.push_str("op: \"");
            result.push_str(extract_operation(content));
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

fn extract_operation(src: &[u8]) -> &str {
    let mut src = &src[1..src.len() - 1];

    for i in 0..src.len() {
        if src[i] == b'"' {
            for j in 0..src.len() {
                let pos = src.len() - j - 1;
                if src[pos] == b'"' {
                    src = &src[i + 1..j];
                    break;
                }
            }
        }
    }

    std::str::from_utf8(src).unwrap()
}
