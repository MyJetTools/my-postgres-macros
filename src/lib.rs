extern crate proc_macro;
use proc_macro::TokenStream;

mod fn_impl_reading;
mod reflection;

use syn;

#[proc_macro_derive(MyPostgresReadSingleRow, attributes(db_field_name,))]
pub fn my_postgres_read_single_row(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_reading::generate_read_single_row(&ast)
}
