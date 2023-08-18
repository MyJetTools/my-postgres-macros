use proc_macro2::TokenStream;

pub fn generate_db_column_name_attribute(fields: &mut Vec<TokenStream>, db_column_name: &str) {
    fields.push(quote::quote! {
        #[db_column_name(#db_column_name)]
    })
}

pub fn generate_sql_type(fields: &mut Vec<TokenStream>, sql_type: &str) {
    fields.push(quote::quote! {
        #[sql_type(#sql_type)]
    })
}
