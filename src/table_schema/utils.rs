use types_reader::StructProperty;

use crate::postgres_struct_ext::PostgresStructPropertyExt;

pub fn filter_table_schema_fields<'s>(
    src_fields: &'s [StructProperty],
) -> Vec<&'s StructProperty<'s>> {
    let mut result = Vec::with_capacity(src_fields.len());

    for itm in src_fields {
        if !itm.has_ignore_table_column() {
            result.push(itm);
        }
    }

    result
}
