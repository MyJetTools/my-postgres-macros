use proc_macro::TokenStream;
use quote::quote;
use types_reader::StructProperty;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    /*
    let fields = match crate::postgres_utils::filter_fields(StructProperty::read(ast)) {
        Ok(result) => result,
        Err(err) => {
            return err.into();
        }
    };


    let select_fields = match super::fn_fill_select_fields::fn_fill_select_fields(&fields) {
        Ok(result) => result,
        Err(err) => vec![err.to_compile_error()],
    };


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

                fn fill_select_fields(sql: &mut String) {
                use my_postgres::sql_select::SelectPartValue;
                #(#select_fields)*
            }

                fn get_order_by_fields() -> Option<&'static str>{
                    #orders_by_fields
                }

                fn get_group_by_fields() -> Option<&'static str>{
                   #group_by_fields
                }

                fn from(db_row: &tokio_postgres::Row) -> Self {
                    use my_postgres::sql_select::FromDbRow;
                    Self{
                     #(#from_fields)*
                    }
                }


     */

    let from_fields: Vec<proc_macro2::TokenStream> = vec![];

    let select_fields: Vec<proc_macro2::TokenStream> = vec![];

    quote! {
        impl my_postgres::sql_select::SelectEntity for #struct_name{
        }
    }
    .into()
}
