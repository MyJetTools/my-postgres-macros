use types_reader::EnumCase;

use super::EnumType;

pub fn fn_to_sql_value(result: &mut String, enum_cases: &[EnumCase], type_name: &EnumType) {
    result.push_str(" match self {");

    let mut i = 0;
    for enum_case in enum_cases {
        result.push_str(enum_case.name.as_str());

        result.push_str(" => ");

        result.push_str("my_postgres::SqlValue::");

        result.push_str(type_name.as_type_name());

        result.push_str("(");

        result.push_str(i.to_string().as_str());
        result.push_str(")");
        result.push(',');
        i += 1;
    }

    result.push('}');
}
