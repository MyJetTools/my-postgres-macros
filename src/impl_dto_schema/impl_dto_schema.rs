use proc_macro2::Ident;
use types_reader::StructProperty;

pub fn impl_dto_schema(struct_name: &Ident, fields: &[StructProperty]) -> proc_macro2::TokenStream {
    let mut result = Vec::new();

    for field in fields {
        let field_name = field.get_field_name_ident();
        let sql_type = get_sql_type(field);
        let is_option = field.ty.is_option();
        result.push(quote::quote! {
            DtoColumn{
                name: #field_name,
                sql_type: #sql_type
                is_primary_key: false,
                is_nullable: #is_option

            }
        });
    }

    quote::quote! {
        impl my_postgres::sql_schema::DtoSchema for #struct_name{
            fn get_columns() -> Vec<DtoColumn>{
                use my_postgres::sql_schema::*;
                vec![#(#result),*]
            }
        }
    }
    .into()
}

fn get_sql_type(field: &StructProperty) -> proc_macro2::TokenStream {
    match &field.ty {
        types_reader::PropertyType::U8 => todo!(),
        types_reader::PropertyType::I8 => todo!(),
        types_reader::PropertyType::U16 => todo!(),
        types_reader::PropertyType::I16 => todo!(),
        types_reader::PropertyType::U32 => todo!(),
        types_reader::PropertyType::I32 => todo!(),
        types_reader::PropertyType::U64 => todo!(),
        types_reader::PropertyType::I64 => todo!(),
        types_reader::PropertyType::F32 => todo!(),
        types_reader::PropertyType::F64 => todo!(),
        types_reader::PropertyType::USize => todo!(),
        types_reader::PropertyType::ISize => todo!(),
        types_reader::PropertyType::String => todo!(),
        types_reader::PropertyType::Str => todo!(),
        types_reader::PropertyType::Bool => todo!(),
        types_reader::PropertyType::DateTime => todo!(),
        types_reader::PropertyType::OptionOf(_) => todo!(),
        types_reader::PropertyType::VecOf(_) => todo!(),
        types_reader::PropertyType::Struct(_, _) => todo!(),
        types_reader::PropertyType::HashMap(_, _) => todo!(),
    }
}
