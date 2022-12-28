use proc_macro2::TokenStream;
use quote::quote;
use types_reader::EnumCase;

pub enum EnumType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
}

impl EnumType {
    pub fn get_func_name(&self) -> proc_macro2::TokenStream {
        match self {
            EnumType::U8 => quote!(to_u8).into(),
            EnumType::I8 => quote!(to_i8).into(),
            EnumType::U16 => quote!(to_u16).into(),
            EnumType::I16 => quote!(to_i16).into(),
            EnumType::U32 => quote!(to_u32).into(),
            EnumType::I32 => quote!(to_i32).into(),
            EnumType::U64 => quote!(to_u64).into(),
            EnumType::I64 => quote!(to_i64).into(),
        }
    }

    pub fn get_return_type_name(&self) -> proc_macro2::TokenStream {
        match self {
            EnumType::U8 => quote!(u8).into(),
            EnumType::I8 => quote!(i8).into(),
            EnumType::U16 => quote!(u16).into(),
            EnumType::I16 => quote!(i16).into(),
            EnumType::U32 => quote!(u32).into(),
            EnumType::I32 => quote!(i32).into(),
            EnumType::U64 => quote!(u64).into(),
            EnumType::I64 => quote!(i64).into(),
        }
    }

    pub fn is_comlient_with_db_name(&self) -> bool {
        match self {
            EnumType::U8 => false,
            EnumType::I8 => false,
            EnumType::U16 => false,
            EnumType::I16 => false,
            EnumType::U32 => false,
            EnumType::I32 => true,
            EnumType::U64 => false,
            EnumType::I64 => true,
        }
    }
}

pub fn generate(ast: &syn::DeriveInput, enum_type: EnumType) -> proc_macro::TokenStream {
    let enum_name = &ast.ident;
    let enum_cases = EnumCase::read(ast);

    let to_func_name = enum_type.get_func_name();

    let type_name = enum_type.get_return_type_name();

    let as_numbered = fn_as_numbered_str(enum_cases.as_slice());

    let from_db_value = fn_from_db_value(enum_cases.as_slice());

    let to_typed_number = fn_to_typed_number(enum_cases.as_slice());

    let from_db_result: TokenStream = if enum_type.is_comlient_with_db_name() {
        quote! {
            #type_name::from_db_value(result)
        }
    } else {
        quote! {
            #type_name::from_db_value(result as #type_name)
        }
    };

    quote! {

        impl #enum_name{
            pub fn #to_func_name(&self)->#type_name{
                match self {
                    #(#to_typed_number)*
                }
            }

            pub fn as_numbered_str(&self)->&'static str {
                #(#as_numbered)*
            }

            pub fn from_db_value(src: #type_name)->Self{
                #(#from_db_value)*
                _ => panic!("Invalid value {}", src)
            }

            fn fill_select_part(sql: &mut String, field_name: &str, sql_type: Option<&str>) {
                sql.push_str(field_name);
            }

        }

        impl<'s> my_postgres::SqlValueWriter<'s> for #enum_name{
            fn write(
                &'s self,
                sql: &mut String,
                params: &mut Vec<my_postgres::SqlValueToWrite<'s>>,
                sql_type: Option<&'static str>,
            ) {
                sql.push_str(self.as_numbered_str());
            }
        }

        impl my_postgres::sql_select::FromDbRow<#enum_name> for #enum_name{
            fn from_db_row(row: &tokio_postgres::Row, name: &str, sql_type: Option<&str>) -> Self{
                let result: #type_name = row.get(name);
                #from_db_result
            }
        }


    }
    .into()
}

fn fn_to_typed_number(enum_cases: &[EnumCase]) -> Vec<TokenStream> {
    let mut result = Vec::with_capacity(enum_cases.len());
    let mut i = 0;
    for enum_case in enum_cases {
        let enum_case_name = enum_case.get_name_ident();
        let no = proc_macro2::Literal::usize_unsuffixed(i);

        result.push(quote! {
            Self::#enum_case_name => #no,
        });

        i += 1;
    }

    result
}

pub fn fn_as_numbered_str(enum_cases: &[EnumCase]) -> Vec<TokenStream> {
    let mut result = Vec::with_capacity(enum_cases.len());
    let mut i = 0;
    for enum_case in enum_cases {
        let enum_case_name = enum_case.get_name_ident();
        let no = i.to_string();
        result.push(
            quote! {
                Self::#enum_case_name => #no
            }
            .into(),
        );

        i += 1;
    }

    result
}

/*
pub fn fn_from_db_row(result: &mut String, enum_type: &EnumType) -> Vec<TokenStream> {
    let result = Vec::with_capacity(capacity);
    result.push_str("impl my_postgres::sql_select::FromDbRow<");
    result.push_str(type_name);
    result.push_str("> for ");
    result.push_str(type_name);
    result.push_str(
        "{fn from_db_row(row: &tokio_postgres::Row, name: &str, sql_type: Option<&str>) -> ",
    );
    result.push_str(type_name);

    result.push_str("{let result: ");
    result.push_str(enum_type.db_complient_type_name());

    result.push_str(" = row.get(");
    result.push_str("name");
    result.push_str(");");

    if enum_type.db_complient_type_name() == enum_type.as_type_name() {
        result.push_str(type_name);
        result.push_str("::from_db_value(result)");
    } else {
        result.push_str(type_name);
        result.push_str("::from_db_value(result as ");
        result.push_str(enum_type.as_type_name());
        result.push(')');
    }

    result.push_str("}}");
}
 */
fn fn_from_db_value(enum_cases: &[EnumCase]) -> Vec<TokenStream> {
    let mut result = Vec::with_capacity(enum_cases.len());

    let mut i = 0;

    for enum_case in enum_cases {
        let no = proc_macro2::Literal::usize_unsuffixed(i);

        let name_ident = enum_case.get_name_ident();

        result.push(quote! {
            #no => Self::#name_ident,
        });
        i += 1;
    }

    result
}
