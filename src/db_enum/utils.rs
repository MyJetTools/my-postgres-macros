use types_reader::EnumCase;

use crate::postgres_enum_ext::PostgresEnumExt;

pub fn render_update_value_provider_fn_body() -> proc_macro2::TokenStream {
    quote::quote! {
        let value = self.to_str();
        let index = params.push(value.into());
        my_postgres::sql::SqlUpdateValue::Index(index)
    }
}

pub fn render_select_part() -> proc_macro2::TokenStream {
    quote::quote! {
        sql.push(my_postgres::sql::SelectFieldValue::Field(field_name));
    }
}

pub fn render_fn_is_none() -> proc_macro2::TokenStream {
    quote::quote! {
        fn is_none(&self) -> bool{
            false
        }
    }
}

pub fn get_default_value(enum_cases: &[EnumCase]) -> Result<proc_macro2::TokenStream, syn::Error> {
    for enum_case in enum_cases {
        if enum_case.attrs.has_attr("default_value") {
            let value = enum_case.get_case_any_string_value()?;

            return Ok(quote::quote! {
            pub fn get_default_value()->&'static str{
              #value
            }
            });
        }
    }

    Ok(quote::quote!())
}
