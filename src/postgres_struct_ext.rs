use types_reader::{attribute_params::ParamValue, PropertyType, StructProperty};

pub const ATTR_PRIMARY_KEY: &str = "primary_key";
pub const ATTR_DB_FIELD_NAME: &str = "db_field_name";
//pub const ATTR_IGNORE_IF_NULL: &str = "ignore_if_null";

pub const ATTR_SQL_TYPE: &str = "sql_type";
pub const ATTR_JSON: &str = "json";

pub trait PostgresStructPropertyExt {
    fn is_primary_key(&self) -> bool;

    fn get_sql_type(&self) -> Result<ParamValue, syn::Error>;

    fn get_db_field_name_as_token(&self) -> proc_macro2::TokenStream;

    fn get_db_field_name_as_string(&self) -> String;

    fn get_model_db_field_name_as_string(&self) -> Option<ParamValue>;

    fn has_json_attr(&self) -> bool;

    fn has_ignore_attr(&self) -> bool;
    fn has_ignore_if_null_attr(&self) -> bool;

    fn is_line_no(&self) -> bool;

    fn sql_value_to_mask(&self) -> bool;

    fn get_field_metadata(&self) -> proc_macro2::TokenStream;
}

impl<'s> PostgresStructPropertyExt for StructProperty<'s> {
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

    fn has_ignore_attr(&self) -> bool {
        self.attrs.has_attr("ignore")
    }

    fn get_sql_type(&self) -> Result<ParamValue, syn::Error> {
        self.attrs.get_single_or_named_param(ATTR_SQL_TYPE, "name")
    }

    fn has_ignore_if_null_attr(&self) -> bool {
        self.attrs.has_attr("ignore_if_null")
    }

    fn has_json_attr(&self) -> bool {
        self.attrs.has_attr(ATTR_JSON)
    }

    fn is_line_no(&self) -> bool {
        self.attrs.has_attr("line_no") || self.name == "line_no"
    }

    fn get_db_field_name_as_token(&self) -> proc_macro2::TokenStream {
        if let Ok(attr) = self.attrs.get_attr(ATTR_DB_FIELD_NAME) {
            if let Ok(result) = attr.get_from_single_or_named("name") {
                let name = result.as_str().to_string();
                return quote::quote!(#name);
            }
        }

        let name = self.name.as_str();

        quote::quote!(#name)
    }

    fn get_db_field_name_as_string(&self) -> String {
        if let Ok(attr) = self
            .attrs
            .get_single_or_named_param(ATTR_DB_FIELD_NAME, "model_field_name")
        {
            return attr.as_str().to_string();
        }

        return self.name.to_string();
    }

    fn get_model_db_field_name_as_string(&self) -> Option<ParamValue> {
        if let Ok(attr) = self
            .attrs
            .get_single_or_named_param(ATTR_DB_FIELD_NAME, "model_field_name")
        {
            return Some(attr);
        }

        return None;
    }

    fn get_field_metadata(&self) -> proc_macro2::TokenStream {
        let model_field_name = self.get_model_db_field_name_as_string();

        let sql_type = self.get_sql_type();

        if model_field_name.is_none() && sql_type.is_err() {
            return quote::quote!(None);
        }

        let model_field_name = if let Some(model_field_name) = model_field_name {
            let model_field_name = model_field_name.as_str();
            quote::quote!(Some(#model_field_name))
        } else {
            quote::quote!(None)
        };

        let sql_type = if let Ok(sql_type) = sql_type {
            let sql_type = sql_type.as_str();
            quote::quote!(Some(#sql_type))
        } else {
            quote::quote!(None)
        };

        quote::quote!({
            Some(my_postgres::SqlValueMetadata{
                sql_type: #sql_type,
                related_field_name: #model_field_name
            })
        })
    }
}

pub fn filter_fields(
    src: Vec<StructProperty>,
) -> Result<Vec<StructProperty>, proc_macro::TokenStream> {
    let mut result = Vec::with_capacity(src.len());

    for itm in src {
        if itm.has_ignore_attr() {
            continue;
        }

        if itm.ty.is_date_time() {
            if let Ok(attr) = itm.get_sql_type() {
                let attr = attr.as_str();
                if attr != "timestamp" && attr != "bigint" {
                    let result = syn::Error::new_spanned(
                        itm.field,
                        format!("Sql type must be 'timestamp' or 'bigint'"),
                    );

                    let err = result.to_compile_error();
                    return Err(quote::quote!(#err).into());
                }
            } else {
                let result = syn::Error::new_spanned(
                    itm.field,
                    format!("Please specify sql_type for {}", itm.name),
                );

                let err = result.to_compile_error();
                return Err(quote::quote!(#err).into());
            }
        }

        result.push(itm);
    }

    return Ok(result);
}
