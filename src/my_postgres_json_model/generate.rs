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

        impl my_postgres::sql_select::SelectPartValue for #struct_name {
            fn fill_select_part(sql: &mut String, field_name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) {
                sql.push_str(field_name);
                sql.push_str(" #>> '{}' as \"");
                sql.push_str(field_name);
                sql.push('"');
            }
        }

        impl my_postgres::sql_select::FromDbRow<#struct_name> for #struct_name {
            fn from_db_row(row: &tokio_postgres::Row, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> #struct_name {
                let str_value: String = row.get(name);
                Self::from_str(str_value.as_str())                
            }

            fn from_db_row_opt(row: &tokio_postgres::Row, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Option<#struct_name> {
                let str_value: Option<String> = row.get(name);
                let str_value = str_value.as_ref()?;
        
                let result = Self::from_str(str_value);
                Some(result)            
            }
        }

        impl<'s> my_postgres::SqlUpdateValueWriter<'s> for #struct_name {
            fn write(
                &'s self,
                sql: &mut String,
                params: &mut Vec<my_postgres::SqlValue<'s>>,
                metadata: &Option<my_postgres::SqlValueMetadata>,
            ) {
                params.push(my_postgres::SqlValue::ValueAsString(self.to_string()));
                sql.push_str("cast($");
                sql.push_str(params.len().to_string().as_str());
                sql.push_str("::text as json)");
            }
        }
    }.into()
}
