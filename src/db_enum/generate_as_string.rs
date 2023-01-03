use quote::quote;
pub fn generate_as_string(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let enum_name = &ast.ident;

    quote! {

        impl #enum_name{
            pub fn to_str(&self)->&'static str {
                todo!("Implement")
            }


            pub fn from_str(src: &str)->Self{
                match src{
                    "" => todo!("Implement"),
                  _ => panic!("Invalid value {}", src)
                }
            }

            fn fill_select_part(sql: &mut String, field_name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) {
                sql.push_str(field_name);
            }

        }

        impl<'s> my_postgres::SqlValueWriter<'s> for #enum_name{
            fn write(
                &'s self,
                sql: &mut String,
                params: &mut Vec<my_postgres::SqlValue<'s>>,
                metadata: &Option<my_postgres::SqlValueMetadata>,
            ) {
                sql.push_str(self.as_numbered_str());
            }

            fn get_default_operator(&self) -> &str{
                "="
            }
        }

        impl my_postgres::sql_select::FromDbRow<#enum_name> for #enum_name{
            fn from_db_row(row: &tokio_postgres::Row, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Self{
                let result: String = row.get(name);
                Self::from_str(result.as_str())
            }
        }


    }
    .into()
}
