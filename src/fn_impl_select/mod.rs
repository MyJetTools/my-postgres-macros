mod fn_fill_group_by;
mod fn_fill_order_by;
mod fn_fill_select_fields;
mod fn_from;
mod generate;
pub use generate::generate;
use proc_macro2::TokenStream;
use quote::quote;
use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;

fn fill_sql_type(prop: &StructProperty) -> TokenStream {
    if let Some(sql_type) = prop.get_sql_type() {
        quote!(Some(#sql_type)).into()
    } else {
        quote!(None).into()
    }
}
