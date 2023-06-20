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

    pub fn get_compliant_with_db_type(&self) -> proc_macro2::TokenStream {
        match self {
            EnumType::U8 => quote!(i32).into(),
            EnumType::I8 => quote!(i32).into(),
            EnumType::U16 => quote!(i32).into(),
            EnumType::I16 => quote!(i32).into(),
            EnumType::U32 => quote!(i32).into(),
            EnumType::I32 => quote!(i32).into(),
            EnumType::U64 => quote!(i64).into(),
            EnumType::I64 => quote!(i32).into(),
        }
    }
}

pub fn generate(ast: &syn::DeriveInput, enum_type: EnumType) -> proc_macro::TokenStream {
    let enum_name = &ast.ident;
    let enum_cases = match EnumCase::read(ast) {
        Ok(cases) => cases,
        Err(e) => return e.to_compile_error().into(),
    };

    let to_func_name = enum_type.get_func_name();

    let type_name = enum_type.get_return_type_name();

    let as_numbered = fn_as_numbered_str(enum_cases.as_slice());

    let from_db_value = fn_from_db_value(enum_cases.as_slice());

    let to_typed_number = fn_to_typed_number(enum_cases.as_slice());

    let sql_db_type = enum_type.get_compliant_with_db_type();

    let from_db_result = if type_name.to_string() == sql_db_type.to_string() {
        quote! {
            Self::from_db_value(result)
        }
    } else {
        quote! {
            Self::from_db_value(result as #type_name)
        }
    };

    let fn_is_none = super::utils::render_fn_is_none();

    let fn_is_none_opt = super::utils::render_fn_is_none_optional();

    let fn_get_default_operator_opt = super::utils::render_fn_get_default_operator_optional();

    quote! {

        impl #enum_name{
            pub fn #to_func_name(&self)->#type_name{
                match self {
                    #(#to_typed_number),*
                }
            }

            pub fn as_numbered_str(&self)->&'static str {
                match self{
                #(#as_numbered),*
                }
            }

            pub fn from_db_value(src: #type_name)->Self{
                match src{
                  #(#from_db_value)*
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
                sql.push_str(self.as_numbered_str());
            }
        }

        impl<'s> my_postgres::SqlWhereValueWriter<'s> for #enum_name{
            fn write(
                &'s self,
                sql: &mut String,
                params: &mut Vec<my_postgres::SqlValue<'s>>,
                metadata: &Option<my_postgres::SqlValueMetadata>,
            ) {
                sql.push_str(self.as_numbered_str());
            }

            fn get_default_operator(&self) -> &str{
                "="
            }

            #fn_is_none
        }

        impl<'s> my_postgres::SqlWhereValueWriter<'s> for Option<#enum_name>{
            fn write(
                &'s self,
                sql: &mut String,
                params: &mut Vec<my_postgres::SqlValue<'s>>,
                metadata: &Option<my_postgres::SqlValueMetadata>,
            ) {
                if let Some(value) = self {
                    sql.push_str(value.as_numbered_str());
                } else {
                    sql.push_str("NULL");
                }
            }

            #fn_get_default_operator_opt
            
            #fn_is_none_opt
        }

        impl my_postgres::sql_select::FromDbRow<#enum_name> for #enum_name{
            fn from_db_row(row: &my_postgres::DbRow, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Self{
                let result: #sql_db_type = row.get(name);
                #from_db_result
            }

            fn from_db_row_opt(row: &my_postgres::DbRow, name: &str, metadata: &Option<my_postgres::SqlValueMetadata>) -> Option<Self>{
                let result: Option<#sql_db_type> = row.get(name);
                let result = result?;
                Some(#from_db_result)
            }
        }

        impl my_postgres::table_schema::SqlTypeProvider for #enum_name {
            fn get_sql_type(
                _metadata: Option<my_postgres::SqlValueMetadata>,
            ) -> my_postgres::table_schema::TableColumnType {
                use my_postgres::table_schema::*;
                #type_name::get_sql_type(None)
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

        result.push(quote!(Self::#enum_case_name => #no));

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
        result.push(quote!(Self::#enum_case_name => #no).into());

        i += 1;
    }

    result
}

fn fn_from_db_value(enum_cases: &[EnumCase]) -> Vec<TokenStream> {
    let mut result = Vec::with_capacity(enum_cases.len());

    let mut i = 0;

    for enum_case in enum_cases {
        let no = proc_macro2::Literal::usize_unsuffixed(i);

        let name_ident = enum_case.get_name_ident();

        result.push(quote! (#no => Self::#name_ident,));
        i += 1;
    }

    result
}
