use quote::quote;
use types_reader::EnumCase;

use crate::postgres_enum_ext::PostgresEnumExt;
pub fn generate_as_string_with_model(ast: &syn::DeriveInput) -> Result<proc_macro::TokenStream, syn::Error> {
    let enum_name = &ast.ident;

    let enum_cases =  EnumCase::read(ast)?;

    let fn_to_str =  generate_fn_to_str(&enum_cases)?;

    let fn_from_str =  generate_fn_from_str(&enum_cases)?;

    let reading_db_model_from_metadata = super::utils::render_reading_db_row_metadata_model();

    let render_sql_writing = super::utils::render_sql_writing();

    let select_part = super::utils::render_select_part();

    let result = quote! {

        impl #enum_name{
            pub fn to_str(&self)->(&'static str, String) {
                match self{
                    #fn_to_str
                }
            }


            pub fn from_str(name: &str, model: &str)->Self{
                match name {
                    #fn_from_str
                  _ => panic!("Invalid value {}", name)
                }
            }

            pub fn fill_select_part(sql: &mut String, field_name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) {
                #select_part
            }
        }

            impl my_postgres::sql_select::FromDbRow<#enum_name> for #enum_name{
                fn from_db_row(row: &my_postgres::DbRow, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Self{
                    let name: String = row.get(name);

                    #reading_db_model_from_metadata

                    Self::from_str(name.as_str(), model.as_str())
                }

                fn from_db_row_opt(row: &my_postgres::DbRow, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Option<Self>{
                    let name: Option<String> = row.get(name);
                    let name = name?;

                    #reading_db_model_from_metadata

                    Some(Self::from_str(name.as_str(), model.as_str()))
                }
            }

            impl<'s> my_postgres::SqlUpdateValueWriter<'s> for #enum_name{
                fn write(
                    &'s self,
                    sql: &mut String,
                    params: &mut Vec<my_postgres::SqlValue<'s>>,
                    metadata: &Option<my_postgres::SqlValueMetadata>,
                ) {
                    #render_sql_writing
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

        let case_value = case.get_case_value()?;

        if case.model.is_none() {
            return Err(syn::Error::new_spanned(
                case_value,
                "Model is not defined for this enum case",
            ));
        }

        let model = case.model.as_ref().unwrap().get_name_ident();

        result.extend(quote! {
            #case_value => Self::#case_ident(#model::from_str(model)),
        });
    }
    Ok(result)
}

fn generate_fn_to_str(enum_cases: &[EnumCase]) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut result = proc_macro2::TokenStream::new();
    for case in enum_cases {
        let case_ident = &case.get_name_ident();

        let case_value = case.get_case_value()?;

        result.extend(quote! {
            Self::#case_ident(model) => (#case_value, model.to_string()),
        });
    }
    Ok(result)
}
