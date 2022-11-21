use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_where_line(result: &mut String, struct_properties: &[StructProperty]) {
    let mut no = 0;

    result.push('"');
    for struct_property in struct_properties {
        if no > 0 {
            result.push_str(" AND ");
        }

        no += 1;

        result.push_str(struct_property.get_db_field_name());
        result.push_str(" = $");
        result.push_str(no.to_string().as_str());
    }

    result.push('"');
}
