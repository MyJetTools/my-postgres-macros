use quote::quote;
use types_reader::TypeName;

pub fn generate(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let type_name = TypeName::new(ast);

    let struct_name = type_name.get_type_name();

    quote! {
        impl my_postgres::sql_select::SelectPartValue for #struct_name {
            fn fill_select_part(sql: &mut String, field_name: &str, _sql_type: Option<&str>) {
                sql.push_str(field_name);
                sql.push_str("::text");
            }
        }

        impl FromDbRow<#struct_name> for #struct_name {
            fn from_db_row(row: &tokio_postgres::Row, name: &str, _sql_type: Option<&str>) -> #struct_name {
                let str_value: String = row.get(name);
                serde_json::from_str(&str_value).unwrap()
            }
        }
    }.into()
}
