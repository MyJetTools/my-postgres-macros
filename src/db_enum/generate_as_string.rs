use quote::quote;
use types_reader::EnumCase;

use crate::postgres_enum_ext::PostgresEnumExt;
pub fn generate_as_string(ast: &syn::DeriveInput) -> Result<proc_macro::TokenStream, syn::Error> {
    let enum_name = &ast.ident;

    let enum_cases = EnumCase::read(ast)?;

    let fn_to_str = generate_fn_to_str(&enum_cases)?;

    let fn_from_str = generate_fn_from_str(&enum_cases)?;

    let fn_is_none = super::utils::render_fn_is_none();

    let default_value_reading = super::utils::get_default_value( enum_cases.as_slice())?;

    let result = quote! {

        impl #enum_name{

            #default_value_reading
            
            pub fn to_str(&self)->&'static str {
                match self{
                    #fn_to_str
                }
            }


            pub fn from_str(src: &str)->Self{
                match src{
                    #fn_from_str
                  _ => panic!("Invalid value {}", src)
                }
            }

            pub fn fill_select_part(sql: &mut my_postgres::sql::SelectBuilder, field_name: &'static str, metadata: &Option<my_postgres::SqlValueMetadata>) {
                sql.push(my_postgres::sql::SelectFieldValue::Field(field_name));
            }
        }

        impl my_postgres::sql_update::SqlUpdateValueProvider for #enum_name{
            fn get_update_value(
                &self,
                params: &mut my_postgres::sql::SqlValues,
                metadata: &Option<my_postgres::SqlValueMetadata>,
            )->my_postgres::sql::SqlUpdateValue {
                let index = params.push_static_str(self.to_str());
                my_postgres::sql::SqlUpdateValue::Index(index)
            }

        }

        impl my_postgres::SqlWhereValueProvider for #enum_name{
            fn get_where_value(
                &self,
                params: &mut my_postgres::sql::SqlValues,
                _metadata: &Option<my_postgres::SqlValueMetadata>,
            )-> my_postgres::sql::SqlWhereValue{
                let index = params.push_static_str(self.to_str());
                my_postgres::sql::SqlWhereValue::Index(index)
            }


            fn get_default_operator(&self) -> &'static str{
               "="
            }

            #fn_is_none
        }


        impl<'s> my_postgres::sql_select::FromDbRow<'s, #enum_name> for #enum_name{
            fn from_db_row(row: &'s my_postgres::DbRow, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Self{
                let result: String = row.get(name);
                Self::from_str(result.as_str())
            }

            fn from_db_row_opt(row: &'s my_postgres::DbRow, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Option<Self>{
                let result: Option<String> = row.get(name);
                let result = result?;
                Some(Self::from_str(result.as_str()))
            }
        }

        impl my_postgres::table_schema::SqlTypeProvider for #enum_name {
            fn get_sql_type(
                _metadata: Option<my_postgres::SqlValueMetadata>,
            ) -> my_postgres::table_schema::TableColumnType {
                my_postgres::table_schema::TableColumnType::Text
            }
        }

    }
    .into();

    Ok(result)
}

fn generate_fn_from_str(enum_cases: &[EnumCase]) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut result = proc_macro2::TokenStream::new();
    for case in enum_cases {
        let case_ident = &case.get_name_ident();

        let case_value = case.get_case_string_value()?;

        result.extend(quote! {
            #case_value => Self::#case_ident,
        });
    }
    Ok(result)
}

fn generate_fn_to_str(enum_cases: &[EnumCase]) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut result = proc_macro2::TokenStream::new();
    for case in enum_cases {
        let case_ident = &case.get_name_ident();

        let case_value = case.get_case_string_value()?;

        result.extend(quote! {
            Self::#case_ident => #case_value,
        });
    }
    Ok(result)
}
