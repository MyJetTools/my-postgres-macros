use types_reader::{PropertyType, StructProperty};

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn get_field_value(result: &mut String, struct_propery: &StructProperty) {
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
    result.push_str("Some(&self.");
    result.push_str(&struct_propery.name);
    result.push_str(")");
}

fn fill_option_of_sql_value(result: &mut String, struct_propery: &StructProperty) {
    result.push_str("if let Some(value) = &self.");
    result.push_str(&struct_propery.name);
    result.push_str("{Some(value)}else{None}");
}

fn fill_option_of_vec_of_value(result: &mut String, struct_propery: &StructProperty) {
    result.push_str("my_postgres::SqlWhereValue::to_in_operator(\"");
    result.push_str(struct_propery.get_db_field_name());
    result.push_str("\", &self.");
    result.push_str(&struct_propery.name);
    result.push_str(")");
}

fn fill_vec_of_sql_value(result: &mut String, struct_propery: &StructProperty) {
    result.push_str("my_postgres::SqlWhereValue::to_in_operator(\"");
    result.push_str(struct_propery.get_db_field_name());
    result.push_str("\", &Some(self.");
    result.push_str(&struct_propery.name);
    result.push_str("))");
}
