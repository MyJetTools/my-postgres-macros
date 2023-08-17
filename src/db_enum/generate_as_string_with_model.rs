use quote::quote;
use types_reader::EnumCase;

use crate::postgres_enum_ext::PostgresEnumExt;
pub fn generate_as_string_with_model(ast: &syn::DeriveInput) -> Result<proc_macro::TokenStream, syn::Error> {
    let enum_name = &ast.ident;

    let enum_cases =  EnumCase::read(ast)?;

    let fn_to_str =  generate_fn_to_str(&enum_cases)?;

    let fn_from_str =  generate_fn_from_str(&enum_cases)?;


    let update_value_provider_fn_body = super::utils::render_update_value_provider_fn_body();

    let select_part = super::utils::render_select_part();


    let result = quote! {

        impl #enum_name{

     
            pub fn to_str(&self)->String {
                match self{
                    #fn_to_str
                }
            }


            pub fn from_str(src: &str)->Self{
                let (name, model) = my_postgres::utils::get_case_and_model(src);
                match name {
                    #fn_from_str
                  _ => panic!("Invalid value {}", name)
                }
            }

            pub fn fill_select_part(sql: &mut my_postgres::sql::SelectBuilder, field_name: &'static str, metadata: &Option<my_postgres::SqlValueMetadata>) {
                #select_part
            }
        }

            impl my_postgres::sql_select::FromDbRow<#enum_name> for #enum_name{
                fn from_db_row(row: &my_postgres::DbRow, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Self{
                    let value: String = row.get(name);
                    Self::from_str(value.as_str())
                }

                fn from_db_row_opt(row: &my_postgres::DbRow, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Option<Self>{
                    let value: Option<String> = row.get(name);
                    let value = value?;

                    Some(Self::from_str(value.as_str()))
                }
            }

            impl my_postgres::sql_update::SqlUpdateValueProvider for #enum_name{
                fn get_update_value(
                    &self,
                    params: &mut my_postgres::sql::SqlValues,
                    metadata: &Option<my_postgres::SqlValueMetadata>,
                )->my_postgres::sql::SqlUpdateValue {
                    #update_value_provider_fn_body
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

        let case_value = case.get_case_string_value()?;

        result.extend(quote! {
            Self::#case_ident(model) => my_postgres::utils::compile_enum_with_model(#case_value, model.to_string().as_str()),
        });
    }
    Ok(result)
}
