use types_reader::{PropertyType, StructProperty};

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn get_field_value(result: &mut String, struct_propery: &StructProperty) {
    match &struct_propery.ty {
        types_reader::PropertyType::U8 => fill_value(result, struct_propery),
        types_reader::PropertyType::I8 => fill_value(result, struct_propery),
        types_reader::PropertyType::U16 => fill_value(result, struct_propery),
        types_reader::PropertyType::I16 => fill_value(result, struct_propery),
        types_reader::PropertyType::U32 => fill_value(result, struct_propery),
        types_reader::PropertyType::I32 => fill_value(result, struct_propery),
        types_reader::PropertyType::U64 => fill_value(result, struct_propery),
        types_reader::PropertyType::I64 => fill_value(result, struct_propery),
        types_reader::PropertyType::F32 => fill_value(result, struct_propery),
        types_reader::PropertyType::F64 => fill_value(result, struct_propery),
        types_reader::PropertyType::String => fill_value(result, struct_propery),
        types_reader::PropertyType::Str => fill_value(result, struct_propery),
        types_reader::PropertyType::DateTime => fill_value(result, struct_propery),
        types_reader::PropertyType::OptionOf(sub_type) => {
            fill_option_of(result, struct_propery, &sub_type)
        }
        _ => panic!("{} is not supported", struct_propery.ty.as_str()),
    }
}

fn fill_option_of(result: &mut String, struct_propery: &StructProperty, sub_type: &PropertyType) {
    match &sub_type {
        types_reader::PropertyType::U8 => fill_option_of_value(result, struct_propery),
        types_reader::PropertyType::I8 => fill_option_of_value(result, struct_propery),
        types_reader::PropertyType::U16 => fill_option_of_value(result, struct_propery),
        types_reader::PropertyType::I16 => fill_option_of_value(result, struct_propery),
        types_reader::PropertyType::U32 => fill_option_of_value(result, struct_propery),
        types_reader::PropertyType::I32 => fill_option_of_value(result, struct_propery),
        types_reader::PropertyType::U64 => fill_option_of_value(result, struct_propery),
        types_reader::PropertyType::I64 => fill_option_of_value(result, struct_propery),
        types_reader::PropertyType::F32 => fill_option_of_value(result, struct_propery),
        types_reader::PropertyType::F64 => fill_option_of_value(result, struct_propery),
        types_reader::PropertyType::String => fill_option_of_value(result, struct_propery),
        types_reader::PropertyType::Str => fill_option_of_value(result, struct_propery),
        types_reader::PropertyType::DateTime => fill_option_of_value(result, struct_propery),
        _ => panic!("{} is not supported", struct_propery.ty.as_str()),
    }
}

fn fill_value(result: &mut String, struct_propery: &StructProperty) {
    result.push_str("my_postgres::SqlValue::Value {value: &self.");
    result.push_str(&struct_propery.name);
    result.push_str(", sql_type: ");

    fill_sql_type(result, struct_propery);

    result.push_str("}");
}

fn fill_option_of_value(result: &mut String, struct_propery: &StructProperty) {
    result.push_str("if let Some(value) = &self.");
    result.push_str(&struct_propery.name);
    result.push_str("{my_postgres::SqlValue::Value {value, sql_type: ");

    fill_sql_type(result, struct_propery);

    result.push_str("}}else{my_postgres::SqlValue::");
    if struct_propery.has_ignore_if_null_attr() {
        result.push_str("Ignore")
    } else {
        result.push_str("Null")
    }

    result.push_str("}");
}

pub fn fill_sql_type(result: &mut String, struct_propery: &StructProperty) {
    if let Some(sql_type) = struct_propery.attrs.try_get("sql_type") {
        if let Some(content) = sql_type.content.as_ref() {
            result.push_str("Some(");
            result.push_str(crate::postgres_utils::extract_attr_value(content));
            result.push_str(")");
            return;
        }
    }

    result.push_str("None");
}
