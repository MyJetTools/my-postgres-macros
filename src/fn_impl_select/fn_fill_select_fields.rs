use types_reader::{PropertyType, StructProperty};

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_fill_select_fields(result: &mut String, fields: &[StructProperty]) {
    result.push_str("use my_postgres::sql_select::SelectPartValue;");

    let mut no = 0;
    for prop in fields {
        if prop.attrs.has_attr("line_no") {
            continue;
        }

        if no > 0 {
            result.push_str("sql.push(',');");
        }

        no += 1;

        if let Some(sql) = prop.attrs.try_get("sql") {
            if let Some(value) = &sql.content {
                result.push_str("sql.push_str(\"");
                result.push_str(crate::postgres_utils::extract_attr_value(value));
                result.push_str("\");");
            } else {
                panic!(
                    "please specify content inside sql attribute for {}",
                    prop.name
                );
            }
        } else {
            if let PropertyType::OptionOf(sub_type) = &prop.ty {
                result.push_str(sub_type.as_str().as_str());
            } else {
                result.push_str(prop.ty.as_str().as_str());
            }

            result.push_str("::fill_select_part(sql, \"");
            result.push_str(prop.get_db_field_name());
            result.push_str("\", ");

            super::fill_sql_type(result, prop);

            result.push_str(");");
        }
    }
}
