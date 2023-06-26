use quote::quote;
use types_reader::TypeName;

pub fn generate(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let type_name = TypeName::new(ast);

    let struct_name = type_name.get_type_name();

    quote! {

        impl #struct_name{
            pub fn from_str(src:&str)->Self{
                serde_json::from_str(src).unwrap()
            }

            pub fn to_string(&self)->String{
                serde_json::to_string(self).unwrap()
            }
        }

        impl my_postgres::sql_select::SelectValueProvider for #struct_name {
            fn fill_select_part(sql: &mut my_postgres::sql::SelectBuilder, field_name: &'static str, metadata: &Option<my_postgres::SqlValueMetadata>) {
                sql.push(my_postgres::sql::SelectFieldValue::Json(field_name));
            }
        }

        impl my_postgres::sql_select::FromDbRow<#struct_name> for #struct_name {
            fn from_db_row(row: &my_postgres::DbRow, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> #struct_name {
                let str_value: String = row.get(name);
                Self::from_str(str_value.as_str())                
            }

            fn from_db_row_opt(row: &my_postgres::DbRow, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Option<#struct_name> {
                let str_value: Option<String> = row.get(name);
                let str_value = str_value.as_ref()?;
        
                let result = Self::from_str(str_value);
                Some(result)            
            }
        }

        impl my_postgres::sql_update::SqlUpdateValueProvider for #struct_name {
            fn get_update_value(
                &self,
                params: &mut my_postgres::sql::SqlValues,
                metadata: &Option<my_postgres::SqlValueMetadata>,
            )->my_postgres::sql::SqlUpdateValue {
                let index = params.push(self.to_string().into());
                my_postgres::sql::SqlUpdateValue::Json(index)
            }
        }

        impl my_postgres::table_schema::SqlTypeProvider for #struct_name {
            fn get_sql_type(
                _metadata: Option<my_postgres::SqlValueMetadata>,
            ) -> my_postgres::table_schema::TableColumnType {
                my_postgres::table_schema::TableColumnType::Json
            }
        }
    }.into()
}
