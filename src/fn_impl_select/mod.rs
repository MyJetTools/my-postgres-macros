mod fn_fill_group_by;
mod fn_fill_order_by;
mod fn_fill_select_fields;
mod fn_from;
mod generate;
pub use generate::generate;
use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;

fn fill_sql_type(result: &mut String, prop: &StructProperty) {
    if let Some(sql_type) = prop.get_sql_type() {
        result.push_str("Some(\"");
        result.push_str(sql_type.as_str());
        result.push_str("\")");
    } else {
        result.push_str("None");
    }
}
