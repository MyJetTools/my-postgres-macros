use crate::reflection::StructProperty;

pub fn read_value(result: &mut String, property: &StructProperty, structure_name: &str) {
    result.push_str("let sql_value =  my_postgres::code_gens::SqlValue::");

    match &property.ty {
        crate::reflection::PropertyType::U8 => {
            result.push_str("U8(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::I8 => {
            result.push_str("I8(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::U16 => {
            result.push_str("U16(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::I16 => {
            result.push_str("I16(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::U32 => {
            result.push_str("U32(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::I32 => {
            result.push_str("I32(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::U64 => {
            result.push_str("U64(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::I64 => {
            result.push_str("I64(");
            result.push_str(structure_name);
            result.push('.');
        }

        crate::reflection::PropertyType::F32 => {
            result.push_str("F32(");
            result.push_str(structure_name);
            result.push('.');
        }

        crate::reflection::PropertyType::F64 => {
            result.push_str("F64(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::USize => {
            result.push_str("USize(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::ISize => {
            result.push_str("ISize(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::String => {
            result.push_str("String(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::Str => {
            result.push_str("String(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::Bool => {
            result.push_str("Bool(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::DateTime => {
            result.push_str("DateTime(");
            result.push_str(structure_name);
            result.push('.');
        }
        crate::reflection::PropertyType::OptionOf(sub_ty) => {
            if sub_ty.is_date_time() {
                result.push_str("DateTime(sql_value);");
            } else {
                result.push_str("String(sql_value.as_str());");
            }

            return;
        }
        crate::reflection::PropertyType::VecOf(_) => {
            panic!("Vec not supported");
        }
        crate::reflection::PropertyType::Struct(_) => {
            panic!("Struct not supported");
        }
    }

    result.push_str(property.name.as_str());

    if let crate::reflection::PropertyType::String = &property.ty {
        result.push_str(".as_str()")
    }

    result.push_str(");");
}
