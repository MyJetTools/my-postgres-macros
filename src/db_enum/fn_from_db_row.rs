use super::EnumType;

pub fn fn_from_db_row(result: &mut String, type_name: &str, enum_type: &EnumType) {
    result.push_str("impl my_postgres::sql_select::FromDbRow<");
    result.push_str(type_name);
    result.push_str("> for ");
    result.push_str(type_name);
    result.push_str(
        "{fn from_db_row(row: &tokio_postgres::Row, name: &str, sql_type: Option<&str>) -> ",
    );
    result.push_str(type_name);

    result.push_str("{let result: ");
    result.push_str(enum_type.db_complient_type_name());

    result.push_str(" = row.get(");
    result.push_str("name");
    result.push_str(");");

    if enum_type.db_complient_type_name() == enum_type.as_type_name() {
        result.push_str("StatusDto::from_db_value(result)");
    } else {
        result.push_str("StatusDto::from_db_value(result as ");
        result.push_str(enum_type.as_type_name());
        result.push(')');
    }

    result.push_str("}}");
}
