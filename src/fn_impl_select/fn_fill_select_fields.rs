use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_fill_select_fields(result: &mut String, fields: &[StructProperty]) {
    result.push_str("use my_postgres::sql_select::SelectPartValue;");

    let mut no = 0;
    for prop in fields {
        if no > 0 {
            result.push_str("sql.push(',');");
        }

        no += 1;

        if let Some(sql) = prop.attrs.try_get("sql") {
            if let Some(value) = &sql.content {
                result.push_str(crate::postgres_utils::extract_attr_value(value));
            } else {
                panic!(
                    "please specify content inside sql attribute for {}",
                    prop.name
                );
            }
        } else {
            result.push_str(prop.ty.as_str().as_str());
            result.push_str("::fill_select_part(sql, \"");
            result.push_str(prop.get_db_field_name());
            result.push_str("\", ");

            if let Some(sql_type) = prop.get_sql_type() {
                result.push_str("Some(\"");
                result.push_str(sql_type.as_str());
                result.push_str(")");
            } else {
                result.push_str("None");
            }

            result.push_str(");");
        }
    }
}
