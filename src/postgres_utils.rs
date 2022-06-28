use crate::reflection::StructProperty;

pub fn generate_field_names<'s, TIter: Iterator<Item = &'s StructProperty>>(
    result: &mut String,
    properties: TIter,
) {
    let mut first = true;
    for prop in properties {
        if !first {
            result.push(',');
        }

        result.push_str(prop.get_db_field_name());

        first = false;
    }
}

pub fn generate_field_values<'s, TIter: Iterator<Item = &'s StructProperty>>(
    result: &mut String,
    properties: TIter,
) -> i32 {
    let mut no = 1;
    let mut first = true;
    for prop in properties {
        if !first {
            result.push(',');
        }

        no = generate_set_value(result, prop, no);
        first = false;
    }

    no
}

pub fn generate_field_names_runtime<'s, TIter: Iterator<Item = &'s StructProperty>>(
    result: &mut String,
    properties: TIter,
) {
    for prop in properties {
        if prop.has_ignore_if_null_attr() {
            result.push_str("if self.");
            result.push_str(prop.name.as_str());
            result.push_str(".is_some(){\n");
        }

        read_value(result, prop);

        result.push_str("sql.append_field(\"");
        result.push_str(prop.get_db_field_name());
        result.push_str("\", sql_value);");

        if prop.has_ignore_if_null_attr() {
            result.push_str("}\n");
        }
    }
}

pub fn generate_where_runtime<'s, TIter: Iterator<Item = &'s StructProperty>>(
    result: &mut String,
    properties: TIter,
) {
    for prop in properties {
        if prop.ty.is_date_time() {
            result.push_str("sql.append_where_raw(\"");
            result.push_str(prop.get_db_field_name());
            result.push_str("\", &");
            result.push_str("self.");
            result.push_str(prop.name.as_str());
            result.push_str("..to_rfc3339().as_str());");
        } else {
            result.push_str("sql.append_where(\"");

            result.push_str(prop.get_db_field_name());
            result.push_str("\", &");
            result.push_str("self.");
            result.push_str(prop.name.as_str());
            result.push_str(");");
        }
    }
}

pub fn generate_set_value(result: &mut String, prop: &StructProperty, mut no: i32) -> i32 {
    if prop.ty.is_date_time() {
        result.push_str("'{");
        result.push_str(prop.name.as_str());
        result.push_str("}'");
    } else {
        result.push_str("$");
        result.push_str(no.to_string().as_str());

        no += 1;
    }

    no
}

pub fn generate_fields_as_params<'s, TIter: Iterator<Item = &'s StructProperty>>(
    result: &mut String,
    fields: TIter,
) {
    for prop in fields {
        if !prop.ty.is_date_time() {
            result.push_str("&self.");
            result.push_str(prop.name.as_str());
            result.push_str(",\n");
        }
    }
}

pub fn generate_date_time_reading<'s, TIter: Iterator<Item = &'s StructProperty>>(
    result: &mut String,
    fields: TIter,
) {
    for prop in fields {
        if prop.ty.is_date_time() {
            result.push(',');
            result.push_str(prop.name.as_str());
            result.push_str(" =self.");
            result.push_str(prop.name.as_str());
            result.push_str(".to_rfc3339()");
        }
    }
}

pub fn has_ignore_if_null_attributes<'s, TIter: Iterator<Item = &'s StructProperty>>(
    fields: TIter,
) -> bool {
    for prop in fields {
        if prop.has_ignore_if_null_attr() {
            return true;
        }
    }

    false
}

pub fn generate_runtime_execution(result: &mut String, fields: &[StructProperty]) {
    result.push_str("let sql_line = sql.get_sql_line(table_name);");

    for field in fields {
        if field.is_debug() {
            result.push_str("println!(\"{}\", sql_line);\n");
        }
    }

    result.push_str("let values_data = sql.get_values_data();");

    result.push_str("client.execute(sql_line.as_str(),values_data).await?;\n");

    result.push_str("Ok(())");
}

pub fn generate_reading_value_from_model_field(result: &mut String, property: &StructProperty) {
    result.push_str("self.");
    result.push_str(property.name.as_str());
    if property.ty.is_date_time() {
        result.push_str(".to_rfc3339();");
    }
}

pub fn read_value(result: &mut String, property: &StructProperty) {
    result.push_str("let sql_value = my_postgres_utils::SqlValue::");

    match &property.ty {
        crate::reflection::PropertyType::U8 => {
            result.push_str("U8(");
        }
        crate::reflection::PropertyType::I8 => {
            result.push_str("I8(");
        }
        crate::reflection::PropertyType::U16 => {
            result.push_str("U16(");
        }
        crate::reflection::PropertyType::I16 => {
            result.push_str("I16(entity.");
        }
        crate::reflection::PropertyType::U32 => {
            result.push_str("U32(entity.");
        }
        crate::reflection::PropertyType::I32 => {
            result.push_str("I32(entity.");
        }
        crate::reflection::PropertyType::U64 => {
            result.push_str("U64(entity.");
        }
        crate::reflection::PropertyType::I64 => {
            result.push_str("I64(entity.");
        }
        crate::reflection::PropertyType::USize => {
            result.push_str("USize(entity.");
        }
        crate::reflection::PropertyType::ISize => {
            result.push_str("ISize(entity.");
        }
        crate::reflection::PropertyType::String => {
            result.push_str("String(entity.");
        }
        crate::reflection::PropertyType::Str => {
            result.push_str("String(entity.");
        }
        crate::reflection::PropertyType::Bool => {
            result.push_str("Bool(entity.");
        }
        crate::reflection::PropertyType::DateTime => {
            result.push_str("DateTime(entity.");
        }
        crate::reflection::PropertyType::OptionOf(_) => {
            result.push_str("String(entity.");
        }
        crate::reflection::PropertyType::VecOf(_) => {
            panic!("Vec not supported");
        }
        crate::reflection::PropertyType::Struct(_) => {
            panic!("Struct not supported");
        }
    }

    result.push_str(property.name.as_str());

    result.push_str(");");
}
