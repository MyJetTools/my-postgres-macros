use proc_macro::TokenStream;
use quote::quote;
use types_reader::StructProperty;

pub fn generate(ast: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &ast.ident;

    let fields = StructProperty::read(ast)?;

    let fields = crate::postgres_struct_ext::filter_fields(fields)?;

    let fn_select_fields = super::fn_fill_select_fields::fn_fill_select_fields(&fields)?;

    let orders_by_fields = match super::fn_fill_order_by::fn_get_order_by_fields(&fields) {
        Ok(result) => result,
        Err(err) => err.to_compile_error(),
    };

    let group_by_fields = match super::fn_fill_group_by::get_group_by_fields(&fields) {
        Ok(result) => result,
        Err(err) => err.to_compile_error(),
    };

    let from_fields = match super::fn_from::fn_from(&fields) {
        Ok(result) => result,
        Err(err) => vec![err.to_compile_error()],
    };

    let result = quote! {
        impl my_postgres::sql_select::SelectEntity for #struct_name{

            fn fill_select_fields(sql: &mut my_postgres::sql::SelectBuilder) {
                use my_postgres::sql_select::SelectValueProvider;
                #(#fn_select_fields)*
            }

            fn get_order_by_fields() -> Option<&'static str>{
                #orders_by_fields
            }

            fn get_group_by_fields() -> Option<&'static str>{
               #group_by_fields
            }

            fn from(db_row: &my_postgres::DbRow) -> Self {
                use my_postgres::sql_select::FromDbRow;
                Self{
                 #(#from_fields)*
                }
            }
        }

    }
    .into();

    Ok(result)
}
