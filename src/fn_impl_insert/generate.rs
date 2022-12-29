use proc_macro::TokenStream;
use quote::quote;
use types_reader::StructProperty;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = match crate::postgres_utils::filter_fields(StructProperty::read(ast)) {
        Ok(result) => result,
        Err(err) => return err,
    };

    let fields_amount = fields.len();

    let get_field_name = fn_get_field_name(&fields);
    let get_field_value = fn_get_field_value(&fields);

    quote! {
        impl<'s> my_postgres::sql_insert::SqlInsertModel<'s> for #name{

            fn get_fields_amount()->usize{
                #fields_amount
            }

            fn get_field_name(no: usize) -> &'static str{
                match no{
                    #(#get_field_name)*
                    _=>panic!("no such field with number {}", no)
                }
            }

            fn get_field_value(&'s self, no: usize) -> my_postgres::SqlValueWrapper<'s>{
                match no{
                    #(#get_field_value)*
                    _=>panic!("no such field with number {}", no)
                }
            }

        }
    }
    .into()
}

pub fn fn_get_field_name(fields: &[StructProperty]) -> Vec<proc_macro2::TokenStream> {
    let mut result = Vec::new();
    for (i, field) in fields.iter().enumerate() {
        let field_name = field.name.as_str();
        result.push(
            quote! {
                #i=>#field_name,
            }
            .into(),
        );
    }
    result
}

pub fn fn_get_field_value(fields: &[StructProperty]) -> Vec<proc_macro2::TokenStream> {
    let mut result = Vec::new();
    for (i, field) in fields.iter().enumerate() {
        let value = crate::get_field_value::render_field_value(field);

        result.push(
            quote! {
                #i => #value,
            }
            .into(),
        );
    }
    result
}
