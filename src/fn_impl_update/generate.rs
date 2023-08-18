use proc_macro::TokenStream;
use types_reader::{StructProperty, TypeName};

use crate::struct_name::StructName;

use super::update_fields::UpdateFields;

pub fn generate(ast: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let type_name = TypeName::new(ast);

    let struct_name = type_name.get_type_name();

    let fields = StructProperty::read(ast)?;

    let fields = crate::postgres_struct_ext::filter_fields(fields)?;

    let update_fields = UpdateFields::new_from_update_model(&fields);

    let main_model = super::generate_derive_model(
        &struct_name,
        StructName::TypeName(&type_name),
        update_fields,
    )?;

    Ok(main_model.into())
}
