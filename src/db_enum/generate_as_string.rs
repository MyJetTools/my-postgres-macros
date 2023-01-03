use quote::quote;
use types_reader::EnumCase;
pub fn generate_as_string(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let enum_name = &ast.ident;

    let enum_cases = match EnumCase::read(ast) {
        Ok(cases) => cases,
        Err(e) => return e.to_compile_error().into(),
    };

    let fn_to_str = generate_fn_to_str(&enum_cases);

    quote! {

        impl #enum_name{
            pub fn to_str(&self)->&'static str {
                match self{
                    #fn_to_str
                }
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
                sql.push_str(self.to_str());
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

fn generate_fn_to_str(enum_cases: &[EnumCase]) -> proc_macro2::TokenStream {
    let mut result = proc_macro2::TokenStream::new();
    for case in enum_cases {
        let case_name = &case.name;
        let case_str = case.name.to_string();
        result.extend(quote! {
            Self::#case_name => #case_str
        });
    }
    result
}
