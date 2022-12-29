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

        impl my_postgres::sql_select::FromDbRow<#struct_name> for #struct_name {
            fn from_db_row(row: &tokio_postgres::Row, name: &str, _sql_type: Option<&str>) -> #struct_name {
                let str_value: String = row.get(name);
                serde_json::from_str(&str_value).unwrap()
            }
        }

        impl<'s> my_postgres::SqlValueWriter<'s> for #struct_name {
            fn write(
                &'s self,
                sql: &mut String,
                params: &mut Vec<my_postgres::sql_value::SqlValue<'s>>,
                metadata: &Option<my_postgres::sql_value::SqlValueMetadata>,
            ) {
                let value = serde_json::to_string(self).unwrap();
                params.push(my_postgres::sql_value::SqlValue::ValueAsString(value));
                sql.push('$');
                sql.push_str(params.len().to_string().as_str());
            }
        
            fn get_default_operator(&self) -> &str {
                "="
            }
        }
    }.into()
}
