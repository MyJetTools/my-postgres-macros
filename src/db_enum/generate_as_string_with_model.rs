use quote::quote;
use types_reader::EnumCase;

use crate::postgre_enum_ext::PostgresEnumExt;
pub fn generate_as_string_with_model(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
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
            pub fn to_str(&self)->(&'static str, &'static str) {
                match self{
                    #fn_to_str
                }
            }


            pub fn from_str(name: &str, model: &str)->Self{
                match name {
                    #fn_from_str
                  _ => panic!("Invalid value {}", src)
                }
            }

            fn fill_select_part(sql: &mut String, field_name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) {
                sql.push_str(field_name);
            }


            fn from_db_row(row: &tokio_postgres::Row, name: &str, model_name: &str) -> Self{
                let name: String = row.get(name);
                let model: String = row.get(model_name);
                Self::from_str(result.as_str(), model.as_str())
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

        let case_value = case.get_case_value();

        result.extend(quote! {
            Self::#case_ident => #case_value,
        });
    }
    Ok(result)
}
