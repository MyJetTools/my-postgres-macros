use crate::reflection::StructProperty;

pub fn fn_insert(result: &mut String, fields: &[StructProperty]) {
    result.push_str("let sql = format!(\"INSERT INTO {table_name} ");

    generate_fields_to_insert(result, fields);

    result.push('"');

    generate_date_time_reading(result, fields);
    result.push_str(");\n");

    result.push_str("client.execute(sql.as_str(),");
    result.push_str("&[");
    generate_fields_as_params(result, fields);
    result.push_str("],).await?;\n");

    result.push_str("Ok(())");
}

fn generate_fields_to_insert(result: &mut String, fields: &[StructProperty]) {
    result.push_str("(");

    for prop in fields {
        result.push_str(prop.get_db_field_name());
        result.push_str(", ");
    }

    result.push_str(") VALUES (");

    let mut no = 0;

    for prop in fields {
        if !prop.ty.is_date_time() {
            result.push_str("'{");
            result.push_str(prop.name.as_str());
            result.push_str("}'");
        } else {
            result.push_str("$");
            result.push_str(no.to_string().as_str());
            result.push_str(", ");
            no += 1;
        }
    }

    result.push_str(")");
}

fn generate_date_time_reading(result: &mut String, fields: &[StructProperty]) {
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

fn generate_fields_as_params(result: &mut String, fields: &[StructProperty]) {
    for prop in fields {
        if !prop.ty.is_date_time() {
            result.push_str("&self.");
            result.push_str(prop.name.as_str());
            result.push_str(",\n");
        }
    }
}
