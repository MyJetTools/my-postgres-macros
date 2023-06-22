use proc_macro2::TokenStream;
use quote::quote;
use types_reader::EnumCase;

use crate::postgres_enum_ext::PostgresEnumExt;

use super::EnumType;

pub fn generate_with_model(ast: &syn::DeriveInput, enum_type: EnumType) -> Result<TokenStream, syn::Error> {
    let enum_name = &ast.ident;

    let enum_cases = EnumCase::read(ast)?;


    let type_name = enum_type.get_return_type_name();

    let fn_to_str = fn_to_str(enum_cases.as_slice())?;

    let fn_to_numbered = fn_to_numbered(enum_cases.as_slice())?;

    let from_db_value =  fn_from_db_value(enum_cases.as_slice())?;

    let sql_db_type = enum_type.get_compliant_with_db_type();

    let reading_db_model_from_metadata = super::utils::render_reading_db_row_metadata_model();

    let select_part = super::utils::render_select_part();


    let from_db_result = if type_name.to_string() == sql_db_type.to_string() {
        quote! {
            Self::from_db_value(result, model.as_str())
        }
    } else {
        quote! {
            Self::from_db_value(result as #type_name, model.as_str())
        }
    };

    let update_value_provider_fn_body = super::utils::render_update_value_provider_fn_body();

    let result = quote! {

        impl #enum_name{

            pub fn to_str(&self)->(&'static str, String) {
                match self{
                #(#fn_to_str),*
                }
            }

            pub fn to_numbered(&self)->(#type_name, String) {
                match self{
                #(#fn_to_numbered),*
                }
            }

            pub fn from_db_value(src: #type_name, model: &str)->Self{
                match src{
                  #(#from_db_value)*
                  _ => panic!("Invalid value {}", src)
                }
            }

            pub fn fill_select_part(sql: &mut my_postgres::sql::SelectBuilder, field_name: &'static str, metadata: &Option<my_postgres::SqlValueMetadata>) {
               #select_part
            }

        }

        impl<'s> my_postgres::sql_update::SqlUpdateValueProvider<'s> for #enum_name{
            fn get_update_value(
                &'s self,
                params: &mut my_postgres::sql::SqlValues<'s>,
                metadata: &Option<my_postgres::SqlValueMetadata>,
            )->my_postgres::sql::SqlUpdateValue<'s> {
                #update_value_provider_fn_body
            }
        }

       

        impl my_postgres::sql_select::FromDbRow<#enum_name> for #enum_name{
            fn from_db_row(row: &my_postgres::DbRow, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Self{
                let result: #sql_db_type = row.get(name);
                #reading_db_model_from_metadata
                #from_db_result
            }

            fn from_db_row_opt(row: &my_postgres::DbRow, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Option<Self>{
                let result: Option<#sql_db_type> = row.get(name);
                let result = result?;
                #reading_db_model_from_metadata
                Some(#from_db_result)
            }
        }


    }
    .into();

    Ok(result)
}

pub fn fn_to_str(enum_cases: &[EnumCase]) -> Result<Vec<TokenStream>, syn::Error> {
    let mut result = Vec::with_capacity(enum_cases.len());

    let mut no = 0;
    for enum_case in enum_cases {
        let enum_case_name = enum_case.get_name_ident();

        no = match enum_case.get_case_number_value()?{
            Some(value) => value,
            None => no+1,
        };
        
        let no = no.to_string();

        result.push(quote!(Self::#enum_case_name(model) => (#no, model.to_string())).into());
    }

    Ok(result)
}

pub fn fn_to_numbered(enum_cases: &[EnumCase]) -> Result<Vec<TokenStream>,  syn::Error>{
    let mut result = Vec::with_capacity(enum_cases.len());

    let mut no= 0;

    for enum_case in enum_cases {
        let enum_case_name = enum_case.get_name_ident();
        no = match enum_case.get_case_number_value()?{
            Some(value) => value,
            None => no+1,
        };
        
        let no = proc_macro2::Literal::i64_unsuffixed(no);

        result.push(quote!(Self::#enum_case_name(model) => (#no, model.to_string())).into());
    }

    Ok(result)
}

fn fn_from_db_value(enum_cases: &[EnumCase]) -> Result<Vec<TokenStream>, syn::Error> {
    let mut result = Vec::with_capacity(enum_cases.len());
    let mut no= 0;

    for enum_case in enum_cases {
        let name_ident = enum_case.get_name_ident();

        if enum_case.model.is_none() {
            return Err(syn::Error::new_spanned(
                enum_case.get_name_ident(),
                "Model is not defined for this enum case",
            ));
        }

        let model = enum_case.model.as_ref().unwrap().get_name_ident();

        no = match enum_case.get_case_number_value()?{
            Some(value) => value,
            None => no+1,
        };
        
        let no = proc_macro2::Literal::i64_unsuffixed(no);
        result.push(quote! (#no => Self::#name_ident(#model::from_str(model)),));
    }

    Ok(result)
}
