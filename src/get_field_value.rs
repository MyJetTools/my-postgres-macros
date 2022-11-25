use types_reader::{PropertyType, StructProperty};

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
    result.push_str(", options: None}");
}

fn fill_option_of_value(result: &mut String, struct_propery: &StructProperty) {
    result.push_str("{value:if let Some(value) = &self.");
    result.push_str(&struct_propery.name);
    result.push_str(
        "{my_postgres::SqlValue::Value {value, options: None}}else{my_postgres::SqlValue::Null}}",
    );
}
