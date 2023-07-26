use quote::ToTokens;
use types_reader::{ParamValue, PropertyType, StructProperty};

use crate::e_tag::ETagData;

pub const ATTR_PRIMARY_KEY: &str = "primary_key";
pub const ATTR_DB_FIELD_NAME: &str = "db_field_name";
//pub const ATTR_IGNORE_IF_NULL: &str = "ignore_if_null";

pub const ATTR_SQL_TYPE: &str = "sql_type";
pub const ATTR_JSON: &str = "json";

pub struct IndexAttr {
    pub id: u8,
    pub index_name: String,
    pub name: String,
    pub is_unique: bool,
    pub order: String,
}

pub trait PostgresStructPropertyExt<'s> {
    fn is_primary_key(&self) -> bool;

    fn get_primary_key_id(&self, last_id: u8) -> Result<Option<u8>, syn::Error>;

    fn get_sql_type(&self) -> Result<&ParamValue, syn::Error>;

    fn get_db_field_name_as_token(&self) -> Result<proc_macro2::TokenStream, syn::Error>;

    fn get_db_field_name_as_string(&self) -> Result<&str, syn::Error>;

    fn get_model_db_field_name_as_string(&self) -> Option<&ParamValue>;

    fn has_json_attr(&self) -> bool;

    fn has_ignore_attr(&self) -> bool;
    fn has_ignore_if_none_attr(&self) -> bool;

    fn is_line_no(&self) -> bool;

    fn sql_value_to_mask(&self) -> bool;

    fn get_field_metadata(&self) -> Result<proc_macro2::TokenStream, syn::Error>;

    fn has_ignore_table_column(&self) -> bool;

    fn get_index_attrs(&self) -> Result<Option<Vec<IndexAttr>>, syn::Error>;

    fn get_e_tag(&'s self) -> Result<Option<ETagData<'s>>, syn::Error>;

    fn get_ty(&self) -> &PropertyType;

    fn get_field_name_ident(&self) -> &syn::Ident;

    fn get_default_if_null_value(&self) -> Result<Option<&str>, syn::Error>;

    fn get_default_value(&self) -> Result<Option<&str>, syn::Error>;

    fn render_field_value(&self, is_update: bool) -> Result<proc_macro2::TokenStream, syn::Error> {
        match &self.get_ty() {
            types_reader::PropertyType::OptionOf(_) => return self.fill_option_of_value(is_update),
            types_reader::PropertyType::Struct(..) => return self.get_value(is_update),
            _ => return self.get_value(is_update),
        }
    }

    fn get_value(&self, is_update: bool) -> Result<proc_macro2::TokenStream, syn::Error> {
        let name = self.get_field_name_ident();

        let metadata = self.get_field_metadata()?;

        let result = if is_update {
            quote::quote! {
                my_postgres::sql_update::SqlUpdateModelValue{
                    value: Some(&self.#name),
                    metadata: #metadata
                }
            }
            .into()
        } else {
            quote::quote! {
                my_postgres::SqlWhereValueWrapper::Value {
                    value: &self.#name,
                    metadata: #metadata
                }
            }
            .into()
        };

        Ok(result)
    }

    fn fill_option_of_value(
        &self,
        is_update: bool,
    ) -> Result<proc_macro2::TokenStream, syn::Error> {
        let prop_name = self.get_field_name_ident();

        let metadata = self.get_field_metadata()?;

        let else_case: proc_macro2::TokenStream = if self.has_ignore_if_none_attr() {
            if is_update {
                quote::quote!(my_postgres::sql_update::SqlUpdateModelValue::Ignore).into()
            } else {
                quote::quote!(my_postgres::sql_update::SqlUpdateModelValue::Ignore).into()
            }
        } else {
            if is_update {
                quote::quote!(my_postgres::sql_update::SqlUpdateModelValue::Null).into()
            } else {
                quote::quote!(my_postgres::sql_update::SqlUpdateModelValue::Null).into()
            }
        };

        let result = if is_update {
            quote::quote! {
               if let Some(value) = &self.#prop_name{
                  my_postgres::sql_update::SqlUpdateModelValue {value: Some(value), metadata: #metadata}
               }else{
                my_postgres::sql_update::SqlUpdateModelValue {value: None, metadata: #metadata}
               }
            }
        } else {
            quote::quote! {
               if let Some(value) = &self.#prop_name{
                  my_postgres::SqlWhereValueWrapper::Value {value, metadata: #metadata}
               }else{
                    #else_case
               }
            }
        };

        Ok(result)
    }
}

impl<'s> PostgresStructPropertyExt<'s> for StructProperty<'s> {
    fn get_field_name_ident(&self) -> &syn::Ident {
        self.get_field_name_ident()
    }

    fn get_ty(&self) -> &PropertyType {
        &self.ty
    }

    fn sql_value_to_mask(&self) -> bool {
        if self.ty.is_string() {
            return true;
        }

        if let PropertyType::OptionOf(sub_ty) = &self.ty {
            if sub_ty.is_string() {
                return true;
            }
        }

        false
    }

    fn is_primary_key(&self) -> bool {
        self.attrs.has_attr(ATTR_PRIMARY_KEY)
    }

    fn get_primary_key_id(&self, last_id: u8) -> Result<Option<u8>, syn::Error> {
        if let Some(value) = self.attrs.try_get_attr(ATTR_PRIMARY_KEY) {
            match value.try_get_single_param() {
                Some(value) => {
                    return Ok(Some(value.get_value("must be value from 0..255".into())?));
                }
                None => {
                    return Ok(Some(last_id + 1));
                }
            }
        }

        Ok(None)
    }

    fn has_ignore_attr(&self) -> bool {
        self.attrs.has_attr("ignore")
    }

    fn get_sql_type(&self) -> Result<&ParamValue, syn::Error> {
        self.attrs.get_single_or_named_param(ATTR_SQL_TYPE, "name")
    }

    fn has_ignore_if_none_attr(&self) -> bool {
        self.attrs.has_attr("ignore_if_none")
    }

    fn has_ignore_table_column(&self) -> bool {
        self.attrs.has_attr("ignore_table_column")
    }

    fn has_json_attr(&self) -> bool {
        self.attrs.has_attr(ATTR_JSON)
    }

    fn is_line_no(&self) -> bool {
        self.attrs.has_attr("line_no") || self.name == "line_no"
    }

    fn get_default_if_null_value(&self) -> Result<Option<&str>, syn::Error> {
        if let Some(attr) = self.attrs.try_get_attr("default_if_null") {
            let result = attr
                .get_from_single_or_named("value")?
                .get_any_value_as_str()?;
            return Ok(Some(result));
        }

        return Ok(None);
    }

    fn get_default_value(&self) -> Result<Option<&str>, syn::Error> {
        if let Some(attr) = self.attrs.try_get_attr("default_value") {
            let result = attr
                .get_from_single_or_named("value")?
                .get_any_value_as_str()?;
            return Ok(Some(result));
        }

        return Ok(None);
    }

    fn get_db_field_name_as_token(&self) -> Result<proc_macro2::TokenStream, syn::Error> {
        if let Ok(attr) = self.attrs.get_attr(ATTR_DB_FIELD_NAME) {
            if let Ok(result) = attr.get_from_single_or_named("name") {
                let name = result.get_any_value_as_str()?.to_token_stream();
                return Ok(name);
            }
        }

        let name = self.name.as_str();

        Ok(quote::quote!(#name))
    }

    fn get_db_field_name_as_string(&self) -> Result<&str, syn::Error> {
        if let Ok(attr) = self
            .attrs
            .get_single_or_named_param(ATTR_DB_FIELD_NAME, "name")
        {
            return Ok(attr.unwrap_as_string_value()?);
        }

        Ok(self.name.as_str())
    }

    fn get_model_db_field_name_as_string(&self) -> Option<&ParamValue> {
        if let Ok(attr) = self
            .attrs
            .get_named_param(ATTR_DB_FIELD_NAME, "model_field_name")
        {
            return Some(attr);
        }

        return None;
    }

    fn get_index_attrs(&self) -> Result<Option<Vec<IndexAttr>>, syn::Error> {
        let attrs = self.attrs.try_get_attrs("db_index");

        if attrs.is_none() {
            return Ok(None);
        }

        let attrs = attrs.unwrap();

        let mut result = Vec::with_capacity(attrs.len());

        for attr in attrs {
            let id = attr
                .get_named_param("id")?
                .get_value("id must be a number 0..255".into())?;

            let index_name = attr
                .get_named_param("index_name")?
                .unwrap_as_string_value()?
                .to_string();
            let is_unique = attr
                .get_named_param("is_unique")?
                .unwrap_as_bool_value()?
                .get_value();

            let order = attr
                .get_named_param("order")?
                .unwrap_as_string_value()?
                .as_str();
            if order != "DESC" && order != "ASC" {
                return Err(syn::Error::new_spanned(
                    attr.get_token_stream(),
                    "order must be DESC or ASC",
                ));
            }

            result.push(IndexAttr {
                id,
                index_name,
                is_unique,
                order: order.to_string(),
                name: self.get_db_field_name_as_string()?.to_string(),
            })
        }

        Ok(Some(result))
    }

    fn get_field_metadata(&self) -> Result<proc_macro2::TokenStream, syn::Error> {
        let model_field_name = self.get_model_db_field_name_as_string();

        let sql_type = self.get_sql_type();

        if model_field_name.is_none() && sql_type.is_err() {
            return Ok(quote::quote!(None));
        }

        let related_column_name = if let Some(model_field_name) = model_field_name {
            let model_field_name = model_field_name.unwrap_as_string_value()?.as_str();
            quote::quote!(Some(#model_field_name))
        } else {
            quote::quote!(None)
        };

        let sql_type = if let Ok(sql_type) = sql_type {
            let sql_type = sql_type.unwrap_as_string_value()?.as_str();
            quote::quote!(Some(#sql_type))
        } else {
            quote::quote!(None)
        };

        Ok(quote::quote!({
            Some(my_postgres::SqlValueMetadata{
                sql_type: #sql_type,
                related_column_name: #related_column_name
            })
        }))
    }

    fn get_e_tag(&'s self) -> Result<Option<ETagData<'s>>, syn::Error> {
        if !self.attrs.has_attr("e_tag") {
            return Ok(None);
        }

        let result = ETagData {
            field_name: self.get_field_name_ident(),
            column_name: self.get_db_field_name_as_string()?,
        };

        Ok(Some(result))
    }
}

pub fn filter_fields(src: Vec<StructProperty>) -> Result<Vec<StructProperty>, syn::Error> {
    let mut result = Vec::with_capacity(src.len());

    for itm in src {
        if itm.has_ignore_attr() {
            continue;
        }

        let is_date_time = match &itm.ty {
            PropertyType::DateTime => true,
            PropertyType::OptionOf(sub_ty) => sub_ty.is_date_time(),
            _ => false,
        };

        if is_date_time {
            if let Ok(attr) = itm.get_sql_type() {
                let attr = attr.unwrap_as_string_value()?.as_str();
                if attr != "timestamp" && attr != "bigint" {
                    let result = syn::Error::new_spanned(
                        itm.field,
                        format!("Sql type must be 'timestamp' or 'bigint'"),
                    );

                    return Err(result);
                }
            } else {
                let result = syn::Error::new_spanned(
                    itm.field,
                    format!("Please specify sql_type for {}", itm.name),
                );

                return Err(result);
            }
        }

        result.push(itm);
    }

    return Ok(result);
}
