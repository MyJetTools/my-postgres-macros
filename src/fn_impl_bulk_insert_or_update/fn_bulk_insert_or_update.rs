use crate::reflection::StructProperty;

pub fn fn_bulk_insert_or_update(result: &mut String, fields: &[StructProperty]) {
    result.push_str("let mut sql = String::new();");
    result.push_str("let mut values_data = Vec::new();");

    result.push_str("for db_entity in entities {");

    result.push_str(
        "let mut sql_insert_or_update = my_postgres_utils::PosrgresInsertOrUpdateBuilder::new();",
    );

    for property in fields {
        read_value(result, property);

        result.push_str("let sql_line = sql.get_sql_line(table_name, pk_name);\n");
    }

    result.push_str("sql.push_str(sql_line.as_str());");
    result.push_str("sql.push(';');");

    result.push_str("}");

    result.push_str("client.execute(sql,values_data).await?;\n");

    result.push_str("Ok(())");
}

fn read_value(result: &mut String, property: &StructProperty) {
    match &property.ty {
        crate::reflection::PropertyType::U8 => {
            add_with_no_quotes(result, property);
        }
        crate::reflection::PropertyType::I8 => {
            add_with_no_quotes(result, property);
        }
        crate::reflection::PropertyType::U16 => {
            add_with_no_quotes(result, property);
        }
        crate::reflection::PropertyType::I16 => {
            add_with_no_quotes(result, property);
        }
        crate::reflection::PropertyType::U32 => {
            add_with_no_quotes(result, property);
        }
        crate::reflection::PropertyType::I32 => {
            add_with_no_quotes(result, property);
        }
        crate::reflection::PropertyType::U64 => {
            add_with_no_quotes(result, property);
        }
        crate::reflection::PropertyType::I64 => {
            add_with_no_quotes(result, property);
        }
        crate::reflection::PropertyType::USize => {
            add_with_no_quotes(result, property);
        }
        crate::reflection::PropertyType::ISize => {
            add_with_no_quotes(result, property);
        }
        crate::reflection::PropertyType::String => {
            add_str(result, property, ".as_str()");
        }
        crate::reflection::PropertyType::Str => {
            add_str(result, property, "");
        }
        crate::reflection::PropertyType::Bool => result.push_str(".to_string()"),
        crate::reflection::PropertyType::DateTime => {
            result.push_str("sql_insert_or_update.append_insert_field_raw_no_quotes(\"");
            result.push_str(property.get_db_field_name());
            result.push_str("\", ");
            result.push_str("db_entity.");
            result.push_str(property.name.as_str());
            result.push_str(".to_rfc3339());");
        }
        crate::reflection::PropertyType::OptionOf(_) => {
            panic!("Option not supported");
        }
        crate::reflection::PropertyType::VecOf(_) => {
            panic!("Vec not supported");
        }
        crate::reflection::PropertyType::Struct(_) => result.push_str(".to_string()"),
    }
}

fn add_with_no_quotes(result: &mut String, property: &StructProperty) {
    result.push_str("sql_insert_or_update.append_insert_field_raw_no_quotes(\"");
    result.push_str(property.get_db_field_name());
    result.push_str("\", ");
    result.push_str("db_entity.");
    result.push_str(property.name.as_str());
    result.push_str(".to_string());");
}

fn add_str(result: &mut String, property: &StructProperty, as_str: &str) {
    result.push_str("sql_insert_or_update.append_insert_field_raw_no_quotes(\"");
    result.push_str(property.get_db_field_name());
    result.push_str("\", ");
    result.push_str("db_entity.");
    result.push_str(property.name.as_str());
    result.push_str(as_str);
    result.push_str(");");
}
