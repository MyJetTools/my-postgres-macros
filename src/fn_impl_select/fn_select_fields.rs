use types_reader::{PropertyType, StructProperty};

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_select_fields(result: &mut String, struct_properties: &[StructProperty]) {
    result.push('"');

    let mut no = 0;
    for struct_property in struct_properties {
        if struct_property.is_line_no() {
            continue;
        }
        if no > 0 {
            result.push(',');
        }

        if let Some(select) = struct_property.attrs.try_get("sql") {
            if let Some(select) = &select.content {
                let value = crate::postgres_utils::extract_attr_value(select);
                result.push_str(value);
                result.push_str(" as ");
                result.push_str(struct_property.get_db_field_name());
            } else {
                panic!(
                    "please specify content inside sql attribute for {}",
                    struct_property.name
                );
            }
        } else {
            generate_read_field(result, struct_property, &struct_property.ty);
        }

        no += 1;
    }

    fn generate_read_field(
        result: &mut String,
        struct_property: &StructProperty,
        ty: &PropertyType,
    ) {
        if struct_property.has_json_attr() {
            result.push_str(struct_property.get_db_field_name());
            result.push_str("::text ");
            fill_casting_to_name(result, struct_property);
            return;
        }

        match ty {
            types_reader::PropertyType::U8 => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::I8 => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::U16 => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::I16 => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::U32 => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::I32 => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::U64 => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::I64 => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::F32 => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::F64 => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::USize => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::ISize => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::String => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::Str => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::Bool => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::DateTime => {
                if let Some(sql_type) = struct_property.get_sql_type() {
                    if sql_type == "timestamp" {
                        result.push_str("(extract(EPOCH FROM ");
                        result.push_str(struct_property.get_db_field_name());
                        result.push_str(") * 1000000)::bigint ");

                        fill_casting_to_name(result, struct_property);
                    } else if sql_type == "bigint" {
                        fill_standard_field(result, struct_property);
                    } else {
                        panic!("Unknown date time type. Property: {} can be either timestamp or bignint", struct_property.name);
                    }
                } else {
                    panic!("Property {} misses an sql_type attr", struct_property.name)
                }
            }
            types_reader::PropertyType::OptionOf(_) => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::VecOf(_) => {
                fill_standard_field(result, struct_property);
            }
            types_reader::PropertyType::Struct(_) => {
                fill_standard_field(result, struct_property);
            }
        }
    }

    result.push('"')
}

fn fill_standard_field(result: &mut String, struct_property: &StructProperty) {
    result.push_str(struct_property.get_db_field_name());
}

fn fill_casting_to_name(result: &mut String, struct_property: &StructProperty) {
    result.push('\\');
    result.push('"');
    result.push_str(struct_property.get_db_field_name());
    result.push('\\');
    result.push('"');
}
