use crate::reflection::{PropertyType, StructProperty};

pub fn read_value(
    result: &mut String,
    property: &StructProperty,
    sub_property: Option<&PropertyType>,
) {
    result.push_str("let sql_value =  my_postgres::code_gens::SqlValue::");

    let ty = if let Some(sub_property) = sub_property {
        sub_property
    } else {
        &property.ty
    };

    match ty {
        crate::reflection::PropertyType::U8 => {
            result.push_str("U8(");

            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }

            result.push_str(");");
        }
        crate::reflection::PropertyType::I8 => {
            result.push_str("I8(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        crate::reflection::PropertyType::U16 => {
            result.push_str("U16(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        crate::reflection::PropertyType::I16 => {
            result.push_str("I16(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        crate::reflection::PropertyType::U32 => {
            result.push_str("U32(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        crate::reflection::PropertyType::I32 => {
            result.push_str("I32(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        crate::reflection::PropertyType::U64 => {
            result.push_str("U64(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        crate::reflection::PropertyType::I64 => {
            result.push_str("I64(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }

        crate::reflection::PropertyType::F32 => {
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }

        crate::reflection::PropertyType::F64 => {
            result.push_str("F64(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        crate::reflection::PropertyType::USize => {
            result.push_str("USize(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        crate::reflection::PropertyType::ISize => {
            result.push_str("ISize(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        crate::reflection::PropertyType::String => {
            result.push_str("String(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
                result.push_str(".as_str()");
            }
            result.push_str(");");
        }
        crate::reflection::PropertyType::Str => {
            result.push_str("String(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        crate::reflection::PropertyType::Bool => {
            result.push_str("Bool(");
            if sub_property.is_some() {
                result.push_str("sql_value");
            } else {
                result.push_str("self.");
                result.push_str(&property.name);
            }
            result.push_str(");");
        }
        crate::reflection::PropertyType::DateTime => {
            if property.has_timestamp_attr() {
                result.push_str("DateTime(");
                if sub_property.is_some() {
                    result.push_str("sql_value");
                } else {
                    result.push_str("self.");
                    result.push_str(&property.name);
                }
                result.push_str(");");
            } else if property.has_bigint_attr() {
                result.push_str("DateTimeAsUnixMicroseconds(");
                if sub_property.is_some() {
                    result.push_str("sql_value");
                } else {
                    result.push_str("self.");
                    result.push_str(&property.name);
                }
                result.push_str(");");
            } else {
                panic!(
                    "Please specify type of datetime (timestamp or bigint) for property {}",
                    property.name
                );
            }
        }
        crate::reflection::PropertyType::OptionOf(sub_ty) => {
            read_value(result, property, Some(sub_ty.as_ref()));
        }
        crate::reflection::PropertyType::VecOf(_) => {
            panic!("Vec not supported");
        }
        crate::reflection::PropertyType::Struct(_) => {
            panic!("Struct not supported");
        }
    }
}
