use types_reader::{PropertyType, StructProperty};

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_from_db_row(result: &mut String, fields: &[StructProperty]) {
    result.push_str("Self {");

    for prop in fields {
        if prop.ty.is_date_time() {
            if prop.has_bigint_attr() {
                result.push_str(prop.name.as_str());
                result.push_str(": DateTimeAsMicroseconds::new(");

                generate_read_db_row_field(result, prop);
                result.push_str("),");
                continue;
            } else if prop.has_timestamp_attr() {
                result.push_str(prop.name.as_str());
                result.push_str(": {");

                result.push_str("let dt: chrono::NaiveDateTime = ");
                generate_read_db_row_field(result, prop);
                result.push_str(";\nDateTimeAsMicroseconds::new(dt.timestamp_millis() * 1000)},\n");
                continue;
            } else {
                panic!("Unknown date time type. Property: {}", prop.name);
            }
        }

        if let PropertyType::Struct(struct_name) = &prop.ty {
            result.push_str(prop.name.as_str());
            result.push_str(": ");

            result.push_str(struct_name);

            result.push_str("::read_from_db(");

            generate_read_db_row_field(result, prop);
            result.push_str("),");
            continue;
        }

        if let PropertyType::OptionOf(sub_ty) = &prop.ty {
            if sub_ty.is_date_time() {
                if prop.has_timestamp_attr() {
                    result.push_str(prop.name.as_str());

                    result.push_str(": {let dt: Option<chrono::NaiveDateTime> = ");
                    generate_read_db_row_field(result, prop);
                    result.push_str(";\n");

                    result.push_str("if let Some(dt)=dt{Some(DateTimeAsMicroseconds::new(dt.timestamp_millis() * 1000))}else{None}},\n");
                    continue;
                } else {
                    result.push_str(prop.name.as_str());
                    result.push_str(": if let Some(value) = ");
                    generate_read_db_row_field(result, prop);
                    result.push_str("{Some(DateTimeAsMicroseconds::new(value))}else{None},");
                    continue;
                }
            }
        }
        result.push_str(prop.name.as_str());
        result.push_str(": ");
        generate_read_db_row_field(result, prop);

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
