use crate::reflection::StructProperty;

pub fn fn_from_db_row(result: &mut String, fields: &[StructProperty]) {
    for prop in fields {
        if prop.ty.is_date_time() {
            result.push_str("let dt: DateTime<Utc> = ");
            generate_read_db_row_field(result, prop);
            result.push_str(";\n");

            ///////////
            result.push_str("let ");
            result.push_str(prop.name.as_str());
            result.push_str(" = DateTimeAsMicroseconds::new(dt.timestamp_millis() * 1000);");
        }
    }

    result.push_str("Self {");

    for prop in fields {
        if prop.ty.is_date_time() {
            result.push_str(prop.name.as_str());
        } else {
            result.push_str(prop.name.as_str());
            result.push_str(": ");
            generate_read_db_row_field(result, prop);
        }

        result.push_str(",\n");
    }

    result.push_str("}\n");
}

fn generate_read_db_row_field(result: &mut String, prop: &StructProperty) {
    result.push_str("row.get(");
    result.push('"');
    result.push_str(prop.get_db_field_name());
    result.push('"');
    result.push(')');
}
