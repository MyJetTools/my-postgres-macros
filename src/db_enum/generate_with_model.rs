use proc_macro2::TokenStream;
use quote::quote;
use types_reader::EnumCase;

use crate::postgres_enum_ext::PostgresEnumExt;

use super::EnumType;

pub fn generate_with_model(ast: &syn::DeriveInput, enum_type: EnumType) -> proc_macro::TokenStream {
    let enum_name = &ast.ident;
    let enum_cases = match EnumCase::read(ast) {
        Ok(cases) => cases,
        Err(e) => return e.to_compile_error().into(),
    };

    let type_name = enum_type.get_return_type_name();

    let fn_to_str = fn_to_str(enum_cases.as_slice());

    let fn_to_numbered = fn_to_numbered(enum_cases.as_slice());

    let from_db_value = match fn_from_db_value(enum_cases.as_slice()) {
        Ok(result) => result,
        Err(e) => return e.to_compile_error().into(),
    };

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

    let render_sql_writing = super::utils::render_sql_writing();

    quote! {

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

            pub fn fill_select_part(sql: &mut String, field_name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) {
               #select_part
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

        impl<'s> my_postgres::SqlWhereValueWriter<'s> for #enum_name{
            fn write(
                &'s self,
                sql: &mut String,
                params: &mut Vec<my_postgres::SqlValue<'s>>,
                metadata: &Option<my_postgres::SqlValueMetadata>,
            ) {
                #render_sql_writing
            }

            fn get_default_operator(&self) -> &str{
                "="
            }
        }        

        impl my_postgres::sql_select::FromDbRow<#enum_name> for #enum_name{
            fn from_db_row(row: &tokio_postgres::Row, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Self{
                let result: #sql_db_type = row.get(name);
                #reading_db_model_from_metadata
                #from_db_result
            }

            fn from_db_row_opt(row: &tokio_postgres::Row, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Option<Self>{
                let result: Option<#sql_db_type> = row.get(name);
                let result = result?;
                #reading_db_model_from_metadata
                Some(#from_db_result)
            }
        }


    }
    .into()
}

pub fn fn_to_str(enum_cases: &[EnumCase]) -> Vec<TokenStream> {
    let mut result = Vec::with_capacity(enum_cases.len());

    for enum_case in enum_cases {
        let enum_case_name = enum_case.get_name_ident();
        let no = enum_case.get_case_value();
        result.push(quote!(Self::#enum_case_name(model) => (#no, model.to_string())).into());
    }

    result
}

pub fn fn_to_numbered(enum_cases: &[EnumCase]) -> Vec<TokenStream> {
    let mut result = Vec::with_capacity(enum_cases.len());

    for enum_case in enum_cases {
        let enum_case_name = enum_case.get_name_ident();
        let no = enum_case.get_case_value();
        let no: TokenStream = no.parse().unwrap();
        result.push(quote!(Self::#enum_case_name(model) => (#no, model.to_string())).into());
    }

    result
}

fn fn_from_db_value(enum_cases: &[EnumCase]) -> Result<Vec<TokenStream>, syn::Error> {
    let mut result = Vec::with_capacity(enum_cases.len());

    for enum_case in enum_cases {
        let name_ident = enum_case.get_name_ident();

        if enum_case.model.is_none() {
            return Err(syn::Error::new_spanned(
                enum_case.get_name_ident(),
                "Please specify #[enum_case(value=\"Value\")]",
            ));
        }

        let model = enum_case.model.as_ref().unwrap().get_name_ident();

        let no = enum_case.get_case_value();
        let no: TokenStream = no.parse().unwrap();

        result.push(quote! (#no => Self::#name_ident(#model::from_str(model)),));
    }

    Ok(result)
}
