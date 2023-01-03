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

            fn fill_select_part(sql: &mut String, field_name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) {
                sql.push_str(field_name);
            }
        }

            impl my_postgres::sql_select::FromDbRow<#enum_name> for #enum_name{
                fn from_db_row(row: &tokio_postgres::Row, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Self{
                    let name: String = row.get(name);

                    let model = if let Some(metadata) = metadata{
                       if metadata.related_field_name.is_none(){
                        panic!("Metadata model field_name is none");
                       }
                       metadata.related_field_name.unwrap()
                    }
                    else{
                        panic!("Metadata is not defined for enum with model");
                    };

                    Self::from_str(name.as_str(), model)
                }
            }

            impl<'s> my_postgres::SqlValueWriter<'s> for #enum_name{
                fn write(
                    &'s self,
                    sql: &mut String,
                    params: &mut Vec<my_postgres::SqlValue<'s>>,
                    metadata: &Option<my_postgres::SqlValueMetadata>,
                ) {
                    let (name, model) = self.to_str();
                    params.push(my_postgres::SqlValue::Ref(*name));
                    sql.push('$');
                    sql.push_str(params.len().to_string().as_str());

                    params.push(my_postgres::SqlValue::ValueAsString(model));
                    sql.push('$');
                    sql.push_str(params.len().to_string().as_str());
                }
    
                fn get_default_operator(&self) -> &str{
                   panic!("Enum with model can not be used in where clause");
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
            Self::#case_ident(model) => (#case_value, model.to_string()),
        });
    }
    Ok(result)
}
