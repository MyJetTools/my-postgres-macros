use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_get_field_value(result: &mut String, struct_properties: &[StructProperty]) {
    let mut no = 1;

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
                result.push_str(".unix_microseconds.to_string()}");
            } else {
                result.push_str("my_postgres::InputDataValue::AsString { name: \"");
                result.push_str(struct_propery.get_db_field_name());
                result.push_str("\", value: self.");
                result.push_str(&struct_propery.name);
                result.push_str(".to_rfc3339()}");
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
    result.push_str("}");
}
