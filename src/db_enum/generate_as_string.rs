use quote::quote;
use types_reader::EnumCase;

use crate::postgres_enum_ext::PostgresEnumExt;
pub fn generate_as_string(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let enum_name = &ast.ident;

    let enum_cases = match EnumCase::read(ast) {
        Ok(cases) => cases,
        Err(e) => return e.to_compile_error().into(),
    };

    let fn_to_str = match generate_fn_to_str(&enum_cases) {
        Ok(result) => result,
        Err(e) => return e.to_compile_error().into(),
    };

    let fn_from_str = match generate_fn_from_str(&enum_cases) {
        Ok(result) => result,
        Err(e) => return e.to_compile_error().into(),
    };

    quote! {

        impl #enum_name{
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

            fn fill_select_part(sql: &mut String, field_name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) {
                sql.push_str(field_name);
            }

        }

        impl<'s> my_postgres::SqlUpdateValueWriter<'s> for #enum_name{
            fn write(
                &'s self,
                sql: &mut String,
                params: &mut Vec<my_postgres::SqlValue<'s>>,
                metadata: &Option<my_postgres::SqlValueMetadata>,
            ) {
                sql.push('\'');
                sql.push_str(self.to_str());
                sql.push('\'');
            }

        }

        impl<'s> my_postgres::SqlWhereValueWriter<'s> for #enum_name{
            fn write(
                &'s self,
                sql: &mut String,
                params: &mut Vec<my_postgres::SqlValue<'s>>,
                metadata: &Option<my_postgres::SqlValueMetadata>,
            ) {
                sql.push('\'');
                sql.push_str(self.to_str());
                sql.push('\'');
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

            fn from_db_row_opt(row: &tokio_postgres::Row, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Option<Self>{
                let result: Option<String> = row.get(name);
                let result = result?;
                Some(Self::from_str(result.as_str()))
            }
        }


    }
    .into()
}

fn generate_fn_from_str(enum_cases: &[EnumCase]) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut result = proc_macro2::TokenStream::new();
    for case in enum_cases {
        let case_ident = &case.get_name_ident();

        let case_value = case.get_case_value();

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

        let case_value = case.get_case_value();

        result.extend(quote! {
            Self::#case_ident => #case_value,
        });
    }
    Ok(result)
}
